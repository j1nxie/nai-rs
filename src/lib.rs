use std::{str::FromStr, string::ParseError};

use bpm::{Bpm, BpmDef};
use met::Met;
use note::NoteType;
use sfl::Sfl;

pub mod bpm;
pub mod met;
pub mod note;
pub mod sfl;

pub enum ParserContext {
    None,
    Version,
    Music,
    SequenceId,
    Difficult,
    Level,
    Creator,
    BpmDef,
    MetDef,
    Resolution,
    ClkDef,
    ProgJudgeBpm,
    ProgJudgeAer,
    Tutorial,
    Bpm,
    Met,
    Sfl,
    Note,
}

impl FromStr for ParserContext {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "VERSION" => Self::Version,
            "MUSIC" => Self::Music,
            "SEQUENCEID" => Self::SequenceId,
            "DIFFICULT" => Self::Difficult,
            "LEVEL" => Self::Level,
            "CREATOR" => Self::Creator,
            "BPM_DEF" => Self::BpmDef,
            "MET_DEF" => Self::MetDef,
            "RESOLUTION" => Self::Resolution,
            "CLK_DEF" => Self::ClkDef,
            "PROGJUDGE_BPM" => Self::ProgJudgeBpm,
            "PROGJUDGE_AER" => Self::ProgJudgeAer,
            "TUTORIAL" => Self::Tutorial,
            "BPM" => Self::Bpm,
            "MET" => Self::Met,
            "SFL" => Self::Sfl,
            "TAP" | "CHR" | "HLD" | "HXD" | "SLD" | "SLC" | "SXC" | "FLK" | "AIR" | "AUR"
            | "AUL" | "AHD" | "ADW" | "ADR" | "ADL" | "ALD" | "MNE" => Self::Note,
            _ => Self::None,
        };

        Ok(result)
    }
}

impl ParserContext {
    pub fn get_section(line: &str) -> Self {
        let context = line.split('\t').nth(0);
        Self::from_str(context.unwrap()).unwrap()
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct ChuniChart {
    pub version: String,
    pub music: usize,
    pub sequence_id: usize,
    pub difficult: usize,
    pub level: f64,
    pub creator: String,
    pub bpm_def: BpmDef,
    pub resolution: usize,
    pub clk_def: usize,
    pub progjudge_bpm: usize,
    pub progjudge_aer: f64,
    pub tutorial: bool,
    pub bpm: Vec<Bpm>,
    pub met: Vec<Met>,
    pub sfl: Vec<Sfl>,
    pub notes: Vec<NoteType>,
}
