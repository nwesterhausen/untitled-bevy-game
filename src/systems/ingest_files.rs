use serde_yaml::{self};

use crate::systems::define_ingest_files::{IngestFile, IngestFileHeader};

/// Read in an ingest file and return the header information from it.
/// This will return None if the file is un-readable or ill-formatted.
pub fn read_file_header(path: &str) -> Option<IngestFileHeader> {
    // Attempt to open the file passed in
    let f = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            log::warn!("Failed to open {}, {}", path,e);
            return None;
        }
    };
    // Attempt to parse the yaml file into a header: Header object
    let scraped: IngestFile = match serde_yaml::from_reader(f) {
        Ok(f) => f,
        Err(e) => {
            log::warn!("Failed to parse {}, {}", path, e);
            return None;
        }
    };
    Some(scraped.header)
}
