//! This crate contains the application to run elite dangerous rusty assistant

mod command_line;

use std::sync::Arc;
use tokio::fs::create_dir_all;
use tokio::signal;
use tokio::task::{JoinError, JoinSet};
use tracing::{debug, error, info, trace};
use elite_dangerous_journal_watcher::{elite_journal_watcher, processor};
use elite_dangerous_journal_watcher::processor::log_event_processor::LogEventProcessor;
use crate::command_line::process_command_line_args;

#[tokio::main]
async fn main() -> Result<(),String> {
    
    let cli_args = process_command_line_args().expect("Unable to parse command line arguments");
    
    trace!("cli args: {cli_args:#?}");
    
    if ! cli_args.journal_dir.exists() {
        let message = "Cannot find journal directory";
        println!("{}", message);
        return Err(String::from(message))
    }
    
    if ! cli_args.working_dir.exists() {
        debug!("working dir does not exist, attempting to create");
        create_dir_all(&cli_args.working_dir).await.expect(format!("could not create working dir {:?}", cli_args.working_dir).as_str());
    }
    
    let mut task_set = JoinSet::new();
    
    let processor = Arc::new(LogEventProcessor::new());
    
    let journal_dir = cli_args.journal_dir.to_path_buf().clone();
    let working_dir = cli_args.working_dir.to_path_buf().clone();
    
    let (terminate_tx, terminate_rx) = futures::channel::oneshot::channel::<()>();
    
    task_set.spawn(async move {
        elite_journal_watcher::start(working_dir, journal_dir, processor, terminate_rx).await;
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
    
    match task_set.join_next().await.expect("Failed to join thread") {
        Ok(_) => {Ok(())}
        Err(e) => { Err(e.to_string()) }
    }
    
} 

