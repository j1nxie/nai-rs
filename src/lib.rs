use bpm::{Bpm, BpmDef};
use met::Met;
use note::NoteType;
use sfl::Sfl;

pub mod bpm;
pub mod met;
pub mod note;
pub mod sfl;

#[derive(Default, Debug, PartialEq)]
pub struct ChuniChart {
    pub version: String,
    pub music: usize,
    pub sequence_id: usize,
    pub difficult: usize,
    pub level: usize,
    pub creator: String,
    pub bpm_def: BpmDef,
    pub resolution: usize,
    pub clk_def: usize,
    pub progjudge_bpm: usize,
    pub progjudge_aer: f64,
    pub tutorial: bool,
    pub bpm: Bpm,
    pub met: Met,
    pub sfl: Sfl,
    pub notes: Vec<NoteType>,
}
