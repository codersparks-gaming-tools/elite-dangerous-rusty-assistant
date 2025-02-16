mod command_line;

use std::sync::Arc;
use tokio::fs::create_dir_all;
use tokio::signal;
use tokio::task::JoinSet;
use tracing::{debug, error, info, trace};
use elite_dangerous_journal_watcher::elite_journal_watcher;
use elite_dangerous_journal_watcher::processor::journal_file_processor::JournalFileProcessor;
use elite_dangerous_rusty_assistant::EliteDangerousRustyAssistant;
use elite_dangerous_rusty_assistant::traits::EDRAComponent;
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
    let edra = EliteDangerousRustyAssistant::new(db_dir).await;
    
    let edra = Arc::new(edra);
    
    let mut task_set = JoinSet::new();
    let journal_dir = cli_args.journal_dir.clone();

    debug!("Starting Journal watcher");

    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel(1024);
    let (terminate_tx, terminate_rx) = tokio::sync::oneshot::channel::<()>();

    
    let processor = Arc::new(JournalFileProcessor::new(event_tx, cli_args.sender_timeout));

    task_set.spawn(async move {
        elite_journal_watcher::start(config_dir, journal_dir, processor, terminate_rx).await;
    });

    info!("Journal watcher started");

    task_set.spawn(async move {
        match signal::ctrl_c().await {
            Ok(_) => {
                info!("Received Ctrl+C");
                terminate_tx.send(()).expect("Failed to send shutdown message");
            }
            Err(e) => {
                info!("Error receiving Ctrl+C: {}", e);
            }
        }
    });

    debug!("Started signal handler");

    let edra_ref_processing = edra.clone();
    task_set.spawn(async move {

        while let Some(event) = event_rx.recv().await {
            debug!("Received event: {:?}", event);

            let event_ref = Arc::new(event);
            match edra_ref_processing.process_event(event_ref).await {
                Ok(_) => {}
                Err(e) => { error!("Failed to process event: {}", e); }
            }
        }
    });

    debug!("Started event processor(s)");
    
    let edra_ref_render = edra.clone();
    task_set.spawn(async move {
        loop {
            edra_ref_render.render().await;
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    });


    match task_set.join_next().await.expect("Failed to join thread") {
        Ok(_) => {Ok(())}
        Err(e) => { Err(e.to_string()) }
    }

}