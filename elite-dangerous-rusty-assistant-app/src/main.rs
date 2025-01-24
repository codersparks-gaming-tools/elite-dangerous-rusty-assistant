//! This crate contains the application to run elite dangerous rusty assistant

mod command_line;

use std::sync::Arc;
use tokio::fs::create_dir_all;
use tokio::signal;
use tracing::{debug, error, info, trace};
use elite_dangerous_journal_watcher::elite_journal_watcher::EliteJournalWatcher;
use elite_dangerous_journal_watcher::processor;
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
    
    let journal_dir = cli_args.journal_dir.to_path_buf().clone();
    let working_dir = cli_args.working_dir.to_path_buf().clone();
    let journal_watcher_task = tokio::spawn(async move {
        let processor_function = Arc::new(processor::log_event_processor);

        let ejw = EliteJournalWatcher::new(working_dir, journal_dir, processor_function).await;

        match signal::ctrl_c().await {
            Ok(()) => {
                info!("CTRL-C pressed, shutting down");
                ejw.terminate();
            }
            Err(e) => {
                error!("Unable to listen for shutdown signal: {}", e);
            }
        }


    });

    info!("Journal watcher started");
    
    journal_watcher_task.await.expect("journal watcher task failed");
    
    Ok(())
} 

