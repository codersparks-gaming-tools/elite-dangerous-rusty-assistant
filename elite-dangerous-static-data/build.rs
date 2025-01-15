use std::{env, fs};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use csv;
use serde::Deserialize;
use convert_case::{Case, Casing};
use serde::de::DeserializeOwned;

fn main() {
    let project_root = std::env::current_dir().unwrap();
    let src_path = project_root.join("src");

    let version_file_path = project_root.join("version.txt");
    fs::write(&version_file_path, "hello world").expect("TODO: panic message");

    //create_enum(&project_root, &src_path, "factionstate","faction_state");
}
