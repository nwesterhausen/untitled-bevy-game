use crate::systems::ingest_files::read_file_header;
use env_logger::Env;
use log::info;

mod systems;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    match read_file_header("sample.yml") {
        Some(h) => {info!("Successfully parsed ingestFile '{}'",h.unique_id)},
        None => {}
    };
    match read_file_header("sample1.yml") {
        Some(h) => {info!("Successfully parsed ingestFile '{}'",h.unique_id)},
        None => {}
    };
    match read_file_header("sample2.yml") {
        Some(h) => {info!("Successfully parsed ingestFile '{}'",h.unique_id)},
        None => {}
    };
}
