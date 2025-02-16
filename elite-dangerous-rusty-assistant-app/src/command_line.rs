use std::path::PathBuf;
use clap::Parser;
use directories::{ProjectDirs, UserDirs};
use tracing::Level;

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

    /// optional timeout for senders, if not supplied this will default to 500ms
    #[arg(short, long, default_value_t = 500)]
    pub(crate) sender_timeout : u64,

    /// Verbosity of application
    #[arg(short, long, action = clap::ArgAction::Count, default_value_t = 0)]
    pub(crate) verbosity: u8,
}

/// The actual struct that the application will use
#[derive(Debug)]
pub struct CliArgs {

    /// The journal directory
    pub journal_dir : PathBuf,

    /// The data directory
    pub data_dir : PathBuf,

    /// The config directory
    pub config_dir : PathBuf,

    /// The sender timeout
    pub sender_timeout : u64,

    /// The log level
    pub log_level : Level,
}

impl From<CliArgsProxy> for CliArgs {
    fn from(value: CliArgsProxy) -> Self {

        let working_dir = match value.working_dir {
            None => {
                let project_dirs = ProjectDirs::from("uk", "codersparks", "edra");
                if project_dirs.is_some() {
                    let pd = project_dirs.unwrap();
                    pd

                } else {
                    let message = "Unable to find the default app data location, please specify the working directory using the -w option";
                    panic!("{}", message);
                }
            }
            Some(pb) => {

                let project_dirs = ProjectDirs::from_path(pb.clone());

                if project_dirs.is_some() {
                    project_dirs.expect("Failed to get project dirs")
                } else {
                    let message = format!("Could not create project dirs from working dir {:?}, please specify the working directory using the -w option", pb);
                    panic!("{}", message);
                }
            }
        };

        let log_level = match value.verbosity {
            0 => Level::WARN,
            1 => Level::INFO,
            2 => Level::DEBUG,
            3 => Level::TRACE ,
            _ => { panic!("Invalid verbosity level") }
        };

        Self {
            journal_dir: value.journal.unwrap(),
            data_dir: working_dir.data_dir().to_path_buf(),
            config_dir: working_dir.config_dir().to_path_buf(),
            sender_timeout: value.sender_timeout,
            log_level
        }
    }
}


pub fn process_command_line_args() -> Result<CliArgs, String> {

    let mut cli = CliArgsProxy::parse();


    if cli.journal.is_none() {

        if let Some(user_dirs) = UserDirs::new() {

            let journal_dir = user_dirs.home_dir().join("Saved Games").join("Frontier Developments").join("Elite Dangerous");
            cli.journal = Some(journal_dir)
        } else {
            let message = "Unable to find the default journal location, please specify the journal directory using the -j option";
            return Err(String::from(message))
        }
    }


    Ok(CliArgs::from(cli))
}