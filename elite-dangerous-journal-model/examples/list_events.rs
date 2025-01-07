use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use tracing::{debug, info, warn};
use elite_dangerous_journal_model::events::fss_signal_discovered::FssSignalType;
use elite_dangerous_journal_model::JournalEvent;


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

    paths.iter().for_each(|path| {
        info!("Processing {}", path);
        let f = File::open(path).expect("Could not open file");
        let reader = BufReader::new(f);

        let it = serde_json::Deserializer::from_reader(reader).into_iter::<JournalEvent>();

        for (event_num, item) in it.enumerate() {
            debug!("Event: {}, Item: {:?}", event_num+1, item);

            if let Ok(event) = item {
                match event {
                    JournalEvent::Other => {
                        unknown_count += 1;
                        warn!("Unknown event: {}", event_num +1);
                    },
                    JournalEvent::FSSSignalDiscovered(event) => {
                        match event.signal_type {
                            FssSignalType::Unknown(signal_type) => {
                                unknown_signal_types.insert(signal_type);
                            },
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    });

    if unknown_count > 0 {
        warn!("Unknown Journal Events found in Journal, count: {}", unknown_count);
    }
    if unknown_signal_types.len() > 0 {
        warn!("Unknown signals found in Journal, count: {}", unknown_signal_types.len());
        warn!("Unknown signals: {:#?}", unknown_signal_types);
    }
}