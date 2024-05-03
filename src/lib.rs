use std::{str::FromStr, string::ParseError};

use bpm::{Bpm, BpmDef};
use met::{Met, MetDef};
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

/// Representation of a CHUNITHM chart.
#[derive(Default, Debug, PartialEq)]
pub struct ChuniChart {
    /// The version of the chart format. `1.12.00` is the latest version as of
    /// current (CHUNITHM LUMINOUS).
    pub version: String,
    /// The ID for the music track. Usually set to 0, as `Music.xml` now contains
    /// it.
    pub music: usize,
    /// The sequence ID of the chart. Usually set to 0.
    pub sequence_id: usize,
    /// The difficulty of the chart. Usually set to 00, as `Music.xml` now
    /// contains it.
    pub difficult: usize,
    /// The internal level of the chart. Usually set to 0.0, as `Music.xml` now
    /// contains it.
    pub level: f64,
    /// The creator of the chart file. This will display on the bottom left
    /// corner of the song card, unless the difficulty is BASIC or ADVANCED.
    pub creator: String,
    /// The default BPM for the chart.
    pub bpm_def: BpmDef,
    /// The default metronome for the chart.
    pub met_def: MetDef,
    /// The resolution of the chart. This represents the length of a measure,
    /// and it is also used in calculating offset in various other places.
    /// Considering a 4/4 metronome, given the default resolution of 384,
    /// the first beat of the measure would have offset 0, the second would be
    /// 96, and so on.
    pub resolution: usize,
    /// Unknown meaning. Always set to 384.
    pub clk_def: usize,
    /// Unknown meaning. Always set to 240.000.
    pub progjudge_bpm: f64,
    /// Unknown meaning. Always set to 0.999.
    pub progjudge_aer: f64,
    /// Designates whether or not the track is a tutorial track.
    pub tutorial: bool,
    /// All BPM designations in the chart.
    pub bpm: Vec<Bpm>,
    /// All time signature designations in the chart.
    pub met: Vec<Met>,
    /// All playfield speed designations in the chart.
    pub sfl: Vec<Sfl>,
    /// All notes in the chart.
    pub notes: Vec<NoteType>,
}

impl ChuniChart {
    /// Takes a [`String`] and parses it into a [`ChuniChart`].
    ///
    /// Returns a [`ParseError`] if any step during
    /// parsing fails.
    pub fn parse(input: String) -> Result<ChuniChart, ParseError> {
        let mut chart = ChuniChart::default();
        let mut current_section: ParserContext;

        for line in input.lines() {
            let line = line.trim();
            if !line.trim().is_empty() {
                current_section = ParserContext::get_section(line);
                match current_section {
                    ParserContext::Version => {
                        let version = line.split('\t').nth(1).unwrap();
                        chart.version = version.to_string();
                    }

                    ParserContext::Music => {
                        let music = line.split('\t').nth(1).unwrap();
                        chart.music = music.parse::<usize>().unwrap();
                    }

                    ParserContext::SequenceId => {
                        let sequence_id = line.split('\t').nth(1).unwrap();
                        chart.sequence_id = sequence_id.parse::<usize>().unwrap();
                    }

                    ParserContext::Difficult => {
                        let difficult = line.split('\t').nth(1).unwrap();
                        chart.difficult = difficult.parse::<usize>().unwrap();
                    }

                    ParserContext::Level => {
                        let level = line.split('\t').nth(1).unwrap();
                        chart.level = level.parse::<f64>().unwrap();
                    }

                    ParserContext::Creator => {
                        let creator = line.split('\t').nth(1).unwrap();
                        chart.creator = creator.to_string();
                    }

                    ParserContext::BpmDef => {
                        let bpm_def = BpmDef::from_str(line).unwrap();
                        chart.bpm_def = bpm_def;
                    }

                    ParserContext::MetDef => {
                        let met_def = MetDef::from_str(line).unwrap();
                        chart.met_def = met_def;
                    }

                    ParserContext::Resolution => {
                        let resolution = line.split('\t').nth(1).unwrap();
                        chart.resolution = resolution.parse::<usize>().unwrap();
                    }

                    ParserContext::ClkDef => {
                        let clk_def = line.split('\t').nth(1).unwrap();
                        chart.clk_def = clk_def.parse::<usize>().unwrap();
                    }

                    ParserContext::ProgJudgeBpm => {
                        let progjudge_bpm = line.split('\t').nth(1).unwrap().trim();
                        chart.progjudge_bpm = progjudge_bpm.parse::<f64>().unwrap();
                    }

                    ParserContext::ProgJudgeAer => {
                        let progjudge_aer = line.split('\t').nth(1).unwrap().trim();
                        chart.progjudge_aer = progjudge_aer.parse::<f64>().unwrap();
                    }

                    ParserContext::Tutorial => {
                        let tutorial = line.split('\t').nth(1).unwrap().parse::<usize>().unwrap();
                        chart.tutorial = tutorial != 0;
                    }

                    ParserContext::Bpm => {
                        let bpm = Bpm::from_str(line).unwrap();
                        chart.bpm.push(bpm);
                    }

                    ParserContext::Met => {
                        let met = Met::from_str(line).unwrap();
                        chart.met.push(met);
                    }

                    ParserContext::Sfl => {
                        let sfl = Sfl::from_str(line).unwrap();
                        chart.sfl.push(sfl);
                    }

                    ParserContext::Note => {
                        let note = NoteType::from_str(line).unwrap();
                        chart.notes.push(note);
                    }

                    ParserContext::None => continue,
                }
            }
        }

        Ok(chart)
    }
}
