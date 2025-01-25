//! This module allows the creation of a watcher to monitor the journal files produced by elite 
//! dangerous. It will then emit events that another application can make use of.
#[deny(missing_docs)]



/// Module contains the code to implement the elite journal file watchers
pub mod elite_journal_watcher;

/// Module contains the code to persist configuration of watchers
pub mod config;
pub mod processor;
