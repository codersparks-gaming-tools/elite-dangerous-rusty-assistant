//! This crate contains the application to run elite dangerous rusty assistant

mod command_line;

use tokio::fs::create_dir_all;
use tracing::{debug, trace};
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
    
    Ok(())
} 

