use serde::{Deserialize, Serialize};

/// The game systems which are defined from ingest files.
#[derive(Debug, Serialize, Deserialize)]
pub enum GameSystems {
    /// The leveling system modifies the leveling curve for characters overall and for classes
    Leveling,
    /// The classes system defines available classes which can be gained through leveling. This also has to tie into what actions level up the class
    Classes,
    /// What magic spells or building blocks exist are defined here
    Magic,
    /// The world constants can be (re-)defined here.
    Physics,
    /// What skills are available, these reference classes
    Skills,
}

/// Each ingest file includes header information about the data in the file.
/// We actually expect the ingest files to be in
#[derive(Debug, Serialize, Deserialize)]
pub struct IngestFileHeader {
    /// A unique identifier for this ingest file.
    pub unique_id: String,
    /// The game system does this ingest file describe/alter/define
    pub system: GameSystems,
    /// If this ingest file must be before another one, list the ingest file which must follow this one here, by unique_id
    pub must_precede: Option<String>,
    /// If this ingest file must be after another one, list the ingest file which must precede this one here, by unique_id
    pub must_follow: Option<String>,
    /// Author of this ingest file
    pub author: String,
    /// Short description of the contents of this ingest file
    pub description: String,
    /// An internal version number, to be able to upgrade automatically
    pub internal_version: u32,
    /// A version string which can be shown to the user
    pub display_version: String,
    /// The target version of the game this is for, to be able to upgrade/replace automatically
    pub valid_game_internal_version: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IngestFile {
    pub header: IngestFileHeader,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillIngestData {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillsIngestFile {
    pub header: IngestFileHeader,
    pub skills: Vec<SkillIngestData>,
}
