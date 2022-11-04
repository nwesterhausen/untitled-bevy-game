use serde::{Deserialize, Serialize};

/// The game systems which can be defined using ingest files.
/// These are not every system in the game, but any system determined to be "modular"
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

/// The basic definition of an Ingest File. This includes a mapping of only the header information
#[derive(Debug, Serialize, Deserialize)]
pub struct IngestFile {
    /// The header describes the content of the file
    pub header: IngestFileHeader,
}

/// An ingest file for Skills includes a header and an array of skills.
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillsIngestFile {
    /// The header describes the content of the file
    pub header: IngestFileHeader,
    /// An array of skill data to be ingested
    pub skills: Vec<SkillIngestData>,
}

/// A Skill has a few key details, but references Classes which should exist (and if they don't, they are ignored)
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillIngestData {
    /// Displayed name of the skill
    pub name: String,
    /// Description of the skill, which should be brief
    pub description: String,
    /// Full description of the skill, including any extra detail
    pub long_description: Option<String>,
    /// Unique identifier for this skill
    pub unique_id: String,
    /// Required classes, listed by their unique IDs
    pub class_requisites: Vec<String>,
    /// Primary classes (if any), which strongly influence gaining this skill
    pub primary_classes: Option<Vec<String>>,
}
