use elite_dangerous_journal_model::events::exploration::fss_signal_discovered::FssSignalType;
use elite_dangerous_journal_model::JournalEvent;
use std::collections::HashSet;
use std::fs::{metadata, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use tracing::{debug, error, info, warn};

fn main() {
    tracing_subscriber::fmt::init();
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    info!("Loading Journal Entries from {}", path);

    let paths = std::env::args().skip(1).collect::<Vec<_>>();

    info!("Paths: {:?}", paths);
    let mut unknown_count = 0;
    let mut unknown_signal_types = HashSet::new();

    paths.iter().for_each(|path_string| {
        let path= PathBuf::from(path_string);
        process_path(&mut unknown_count, &mut unknown_signal_types, &path)
    });

    if unknown_count > 0 {
        warn!(
            "Unknown Journal Events found in Journal, count: {}",
            unknown_count
        );
    }
    if unknown_signal_types.len() > 0 {
        warn!(
            "Unknown signals found in Journal, count: {}",
            unknown_signal_types.len()
        );
        warn!("Unknown signals: {:#?}", unknown_signal_types);
    }
}

fn process_path(unknown_count: &mut i32, unknown_signal_types: &mut HashSet<String>, path: &PathBuf) {

    let metadata = metadata(path).unwrap();

    if metadata.is_dir() {
        for entry in std::fs::read_dir(path).unwrap() {
            if let Ok(entry) = entry {
                process_path(unknown_count, unknown_signal_types, &entry.path());
            }
        }
    } else {

        if ! path.file_name().unwrap().to_str().unwrap().starts_with("Journal.") {
            debug!("Skipping path: {:?}", path);
            return
        }
        info!("Processing {}", path.display());


        let f = File::open(path).expect("Could not open file");
        let reader = BufReader::new(f);

        let lines = reader.lines();

        //let it = serde_json::Deserializer::from_reader(reader).into_iter::<JournalEvent>();

        for (event_num, line) in lines.enumerate() {

            let line = line.expect("Could not read line");

            let item = serde_json::from_str::<JournalEvent>(&line);
            match item {
                Ok(event) => {

                            debug!("Event: {}, event: {:?}", event_num + 1, event);

                            match event {
                                JournalEvent::Unknown => {
                                    *unknown_count += 1;
                                    warn!("Unknown event {}", event_num + 1);
                                    warn!("\t\t=>{:?}", line);
                                }
                                JournalEvent::FSSSignalDiscovered(event) => match event.signal_type {
                                    FssSignalType::Unknown(signal_type) => {
                                        unknown_signal_types.insert(signal_type);
                                    }
                                    _ => {}
                                },
                                _ => {}
                            }
                        }

                Err(e) => {
                    error!("{e} - line {line}");
                }
            }
        }
    }
}
