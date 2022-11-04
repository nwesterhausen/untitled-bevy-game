use serde_yaml::{self};
use walkdir::WalkDir;

use crate::systems::define_ingest_files::{
    GameSystems, IngestFile, IngestFileHeader, SkillsIngestFile,
};

/// Read in an ingest file and return the header information from it.
/// This will return None if the file is un-readable or ill-formatted.
/// What this does return on success is the FileHeader, mainly the unique ID for the file,
/// the version info, the system it has data for, author, and description.
pub fn read_file_header(path: &str) -> Option<IngestFileHeader> {
    // Attempt to open the file passed in
    let f = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            log::warn!("Failed to open {}, {}", path, e);
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

/// Read in an ingest file and return it. This is generic because all ingest files
/// are the same, they only differ in the struct that is returned.
pub fn read_ingest_file<T: serde::de::DeserializeOwned>(path: &str) -> Option<T> {
    // Attempt to open the file passed in
    let f = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            log::warn!("Failed to open {}, {}", path, e);
            return None;
        }
    };
    // Attempt to parse the yaml file into a header: Header object
    let ingest: T = match serde_yaml::from_reader(f) {
        Ok(f) => f,
        Err(e) => {
            log::warn!("Failed to parse {}, {}", path, e);
            return None;
        }
    };
    Some(ingest)
}

/// Reading in the directory of ingest files:
/// 1. Should be recursive to get all subdirs
/// 2. Should organize the files before reading them all in
///     i.   Read headers for all files
///     ii.  Discard any that are for wrong game version
///     iii. Store [filename, header] in a list for each system
///     iv.  Sort the ingest data lists using any specified ordinal constraints
/// 3. Add the data to the database in system load order (TDB)
/// 4. Validate skill -> class, magic -> skill,class, and other relationships are valid
///
pub fn load_ingest_file_dir(path: &str) {
    let mut skills = Vec::<(&str, SkillsIngestFile)>::new();

    let mut possible_ingests: Vec<String> = WalkDir::new(path)
        .into_iter()
        .filter_map(|file| file.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|e| e.path().to_string_lossy().to_string())
        .collect();

    for d in possible_ingests.iter_mut() {
        let filepath = d.as_str();
        let h = read_file_header(filepath);
        if let Some(header) = h {
            log::info!("Read header of {} from {}", header.unique_id, &filepath);
            match header.system {
                GameSystems::Skills => {
                    if let Some(skilldata) = read_ingest_file::<SkillsIngestFile>(&filepath) {
                        skills.push((&filepath, skilldata));
                    }
                }
                _ => {
                    log::info!("No match for {:?}", header.system);
                }
            }
        }
    }

    log::info!("Skills ingest files contains {} files", skills.len());
}
