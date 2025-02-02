//! This crate contains the application to run elite dangerous rusty assistant

mod command_line;

use std::sync::Arc;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;
use tokio::fs::create_dir_all;
use tokio::signal;
use tokio::sync::mpsc::channel;
use tokio::task::JoinSet;
use tracing::{debug, error, info, trace};
use edra::EliteDangerousEventProcessor;
use edra::pirate_massacre_helper::PirateMassacreHelper;
use elite_dangerous_journal_watcher::elite_journal_watcher;
use elite_dangerous_journal_watcher::processor::journal_file_processor::JournalFileProcessor;
use crate::command_line::process_command_line_args;

#[tokio::main]
async fn main() -> Result<(),String> {
    
    let cli_args = process_command_line_args().expect("Unable to parse command line arguments");
    
    let log_level = cli_args.log_level;
    
    if ! cli_args.journal_dir.exists() {
        let message = "Cannot find journal directory";
        println!("{}", message);
        return Err(String::from(message))
    }

    let config_dir = cli_args.config_dir.to_path_buf().clone();
    if ! config_dir.exists() {
        debug!("Config dir does not exist, attempting to create");
        create_dir_all(&config_dir).await.expect(format!("could not create config dir {:?}", &config_dir).as_str());
    }

    let data_dir = cli_args.data_dir.to_path_buf().clone();
    if ! data_dir.exists() {
        debug!("Data dir does not exist, attempting to create");
        create_dir_all(&data_dir).await.expect(format!("could not create data dir {:?}", &data_dir).as_str());
    }

    let log_dir = data_dir.join("logs");
    let file_appender = tracing_appender::rolling::daily(log_dir, "edra.log");
    
    // We have to actually name the guard variable to allow it to only be dropped at the end of main
    // see https://docs.rs/tracing-appender/latest/tracing_appender/non_blocking/struct.WorkerGuard.html
    // for more details
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)          // Remove the colour
        .with_max_level(log_level)      // Set the max log level to show
        .init();
    
    trace!("cli args: {cli_args:#?}");

    let db_dir = data_dir.join("db");
    
    let db : Surreal<Db> = Surreal::new::<RocksDb>(db_dir).await.expect("Failed to create surreal db handle");
    
    let db_ref = Arc::new(db);
    let pmm = Arc::new(PirateMassacreHelper::new(db_ref.clone()).await);
    
    let helpers = vec![pmm.clone()];


    let mut task_set = JoinSet::new();
    let journal_dir = cli_args.journal_dir.to_path_buf().clone();

    
    
    debug!("Staring journal watcher");
    
    let (event_tx, mut event_rx) = channel(1024);
    let processor = Arc::new(JournalFileProcessor::new(event_tx, cli_args.sender_timeout));

    let (terminate_tx, terminate_rx) = futures::channel::oneshot::channel::<()>();

    task_set.spawn(async move {
        elite_journal_watcher::start(config_dir, journal_dir, processor, terminate_rx).await;
    });

    info!("Journal watcher started");

    task_set.spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                info!("CTRL-C pressed, shutting down");
                terminate_tx.send(()).expect("Failed to send shutdown message");
            }
            Err(e) => {
                error!("Unable to listen for shutdown signal: {}", e);
            }
        }
    });
    
    task_set.spawn(async move {
        
        while let Some(event) = event_rx.recv().await {
            info!("{:?}", event);
            
            let event_ref = Arc::new(event);
            for helper in helpers.iter() {
                helper.process_event(event_ref.clone()).await.expect("Failed to process event");
            }
            
            
        }
    });
    
    let pmm_ref = pmm.clone();
    task_set.spawn(async move {
        loop {
            let missions = pmm_ref.get_mission_summary_by_faction().await;
            
            match missions {
                Ok(missions) => {
                    println!("--------------- Missions Count: {} -------------------", missions.len());

                    missions.iter().for_each(|s| {
                        let ((faction, target_system, target_faction), count) = s;
                        println!("Target System: {}, Target Faction: {}, Faction: {}, Count {}", target_system, target_faction, faction, count );
                    });
                    println!("------------------------------------------------------");
                }
                Err(e) => {  error!("Failed to get missions: {}", e); }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    });

    match task_set.join_next().await.expect("Failed to join thread") {
        Ok(_) => {Ok(())}
        Err(e) => { Err(e.to_string()) }
    }

}