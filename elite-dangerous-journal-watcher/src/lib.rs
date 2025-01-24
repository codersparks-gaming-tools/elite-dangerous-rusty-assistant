//! This module allows the creation of a watcher to monitor the journal files produced by elite 
//! dangerous. It will then emit events that another application can make use of.
#[deny(missing_docs)]


/// The file_watcher module that handles the use of notify crate for watching file changes
pub mod file_watcher;
/// Module contains the code to implement the elite journal file watchers
pub mod elite_journal_watcher;

/// Module contains the code to persist configuration of watchers
mod config;
pub mod processor;
