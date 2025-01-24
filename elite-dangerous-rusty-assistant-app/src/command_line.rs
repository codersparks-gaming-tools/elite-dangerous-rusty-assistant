use std::path::PathBuf;
use clap::Parser;
use directories::{ProjectDirs, UserDirs};
use tracing::{debug, error, Level};

/// edra is a rust implemented assistant for Elite Dangerous
/// At the moment it only parses the journel event files (and not all events have been implemented)
#[derive(Parser, Debug)]
#[command(version, about)]
struct CliArgsProxy {

    /// Optional journal directory location, if not supplied will attempt to find the default installed location
    #[arg(short, long)]
    pub(crate) journal : Option<PathBuf>,
    
    /// optional working directory location, if not supplied this will use the standard app data location
    #[arg(short, long)]
    pub(crate) working_dir : Option<PathBuf>,
    
    /// Verbosity of application
    #[arg(short, long, action = clap::ArgAction::Count, default_value_t = 0)]
    verbosity: u8,
}

/// The actual struct that the application will use
#[derive(Debug)]
pub struct CliArgs {
    
    /// The journal directory
    pub journal_dir : PathBuf,
    
    /// The working directory
    pub working_dir : PathBuf,
}
 
impl From<CliArgsProxy> for CliArgs {
    fn from(value: CliArgsProxy) -> Self {
        Self {
            journal_dir: value.journal.unwrap(),
            working_dir: value.working_dir.unwrap()
        }
    }
}


pub fn process_command_line_args() -> Result<CliArgs, String> {

    let mut cli = CliArgsProxy::parse();
    
    let log_level;
    match cli.verbosity {
        0 => log_level = Level::WARN,
        1 => log_level = Level::INFO,
        2 => log_level = Level::DEBUG,
        3 => log_level = Level::TRACE,
        _ => { panic!("Invalid verbosity level") }
    }
    
    tracing_subscriber::fmt().with_max_level(log_level).init();

    if cli.journal.is_none() {
        debug!("journal dir not supplied, attempting to find default location");

        if let Some(user_dirs) = UserDirs::new() {

            let journal_dir = user_dirs.home_dir().join("Saved Games").join("Frontier Developments").join("Elite Dangerous");
            cli.journal = Some(journal_dir)
        } else {
            let message = "Unable to find the default journal location, please specify the journal directory using the -j option";
            error!("{}", message);
            return Err(String::from(message))
        }
    }
    
    let project_dirs = ProjectDirs::from("uk", "codersparks", "edra");

    if cli.working_dir.is_none() {
        if project_dirs.is_some() {
            let working_dir = project_dirs.unwrap().data_dir().to_path_buf();
            debug!("working dir not supplied, attempting to use the default app data location: {working_dir:?}");
            cli.working_dir = Some(working_dir);
        } else {
            let message = "Unable to find the default app data location, please specify the working directory using the -w option";
            error!("{}", message);
            return Err(String::from(message))
        }
    }

    Ok(CliArgs::from(cli))
}