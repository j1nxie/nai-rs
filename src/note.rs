#![allow(clippy::tabs_in_doc_comments)]
use std::{str::FromStr, string::ParseError};

/// Tap notes are the most basic notes that can be charted. They simply require
/// the player to hit the cell that the note occupies at the required time.
///
/// Tap notes also represent the universal note schema.
#[derive(Debug, Default, PartialEq)]
pub struct Tap {
    /// The specific measure the note will be placed in.
    pub measure: usize,
    /// The offset of the note from the start of the specified measure.
    /// calculated with the method described in [`ChuniChart`][crate::ChuniChart]'s `resolution` field.
    pub offset: usize,
    /// The numerical ID for the column in the playfield that the note should
    /// appear in, ranging between 0-15, with 0 being the leftmost column, and
    /// 15 being the rightmost column.
    pub cell: usize,
    /// The width of the note, extending from the specified cell to the right.
    /// Minimum value is 1, which means that the note only occupies the column
    /// specified.
    pub width: usize,
}

/// ExTaps are the same as Taps, but they will always be judged as a CRITICAL
/// JUSTICE when hit.
#[derive(Debug, Default, PartialEq)]
pub struct ExTap {
    /// The specific measure the note will be placed in.
    pub measure: usize,
    /// The offset of the note from the start of the specified measure.
    /// calculated with the method described in [`ChuniChart`][crate::ChuniChart]'s `resolution` field.
    pub offset: usize,
    /// The numerical ID for the column in the playfield that the note should
    /// appear in, ranging between 0-15, with 0 being the leftmost column, and
    /// 15 being the rightmost column.
    pub cell: usize,
    /// The width of the note, extending from the specified cell to the right.
    /// minimum value is 1, which means that the note only occupies the column
    /// specified.
    pub width: usize,
    /// The animation that is played when an ExTap is hit. Possible values are:
    /// - UP: Vertical effects from bottom to top.
    /// - DW: Vertical effects from top to bottom.
    /// - CE: Effects towards the playfield.
    /// - LS: Horizontal effects from right to left.
    /// - RS: Horizontal effects from left to right.
    /// - LC: Effects rotate counter-clockwise.
    /// - RC: Effects rotate clockwise.
    pub animation: String,
}

/// Hold notes are similar to tap notes, but the player must keep the designated
/// cell pressed over a continuous amount of time.
#[derive(Debug, Default, PartialEq)]
pub struct Hold {
    /// The specific measure the note will be placed in.
    pub measure: usize,
    /// The offset of the note from the start of the specified measure.
    /// calculated with the method described in [`ChuniChart`][crate::ChuniChart]'s `resolution` field.
    pub offset: usize,
    /// The numerical ID for the column in the playfield that the note should
    /// appear in, ranging between 0-15, with 0 being the leftmost column, and
    /// 15 being the rightmost column.
    pub cell: usize,
    /// The width of the note, extending from the specified cell to the right.
    /// minimum value is 1, which means that the note only occupies the column
    /// specified.
    pub width: usize,
    /// The amount of time that the note needs to be held down for. Uses the
    /// same calculation method as offset values.
    pub duration: usize,
    /// The animation that is played when a HoldWithExTapHead is hit. Possible values are:
    /// - UP: Vertical effects from bottom to top.
    /// - DW: Vertical effects from top to bottom.
    /// - CE: Effects towards the playfield.
    /// - LS: Horizontal effects from right to left.
    /// - RS: Horizontal effects from left to right.
    /// - LC: Effects rotate counter-clockwise.
    /// - RC: Effects rotate clockwise.
    ///
    /// Should always be represented with `None` if it is a normal Hold.
    pub animation: Option<String>,
}

/// Introduced in LUMINOUS, these Holds have an ExTap as their head, instead of
/// being notated as two notes: a Hold with an ExTap on top of it.
pub type HoldWithExTapHead = Hold;

#[derive(Debug, Default, PartialEq)]
pub struct Slide {
    /// The specific measure the note will be placed in.
    pub measure: usize,
    /// The offset of the note from the start of the specified measure.
    /// calculated with the method described in [`ChuniChart`][crate::ChuniChart]'s `resolution` field.
    pub offset: usize,
    /// The numerical ID for the column in the playfield that the note should
    /// appear in, ranging between 0-15, with 0 being the leftmost column, and
    /// 15 being the rightmost column.
    pub cell: usize,
    /// The width of the note, extending from the specified cell to the right.
    /// minimum value is 1, which means that the note only occupies the column
    /// specified.
    pub width: usize,
    pub duration: usize,
    pub end_cell: usize,
    pub end_width: usize,
    pub animation: Option<String>,
}

pub type SlideControlPoint = Slide;
pub type SlideWithExTapHead = Slide;
pub type SlideControlPointWithExTapHead = Slide;

#[derive(Debug, Default, PartialEq)]
pub struct Flick {
    /// The specific measure the note will be placed in.
    pub measure: usize,
    /// The offset of the note from the start of the specified measure.
    /// calculated with the method described in [`ChuniChart`][crate::ChuniChart]'s `resolution` field.
    pub offset: usize,
    /// The numerical ID for the column in the playfield that the note should
    /// appear in, ranging between 0-15, with 0 being the leftmost column, and
    /// 15 being the rightmost column.
    pub cell: usize,
    /// The width of the note, extending from the specified cell to the right.
    /// minimum value is 1, which means that the note only occupies the column
    /// specified.
    pub width: usize,
    pub unknown: String,
}

#[derive(Debug, Default, PartialEq)]
pub struct Air {
    /// The specific measure the note will be placed in.
    pub measure: usize,
    /// The offset of the note from the start of the specified measure.
    /// calculated with the method described in [`ChuniChart`][crate::ChuniChart]'s `resolution` field.
    pub offset: usize,
    /// The numerical ID for the column in the playfield that the note should
    /// appear in, ranging between 0-15, with 0 being the leftmost column, and
    /// 15 being the rightmost column.
    pub cell: usize,
    /// The width of the note, extending from the specified cell to the right.
    /// minimum value is 1, which means that the note only occupies the column
    /// specified.
    pub width: usize,
    pub target_note: String,
}

pub type AirUpRight = Air;
pub type AirUpLeft = Air;
pub type AirDown = Air;
pub type AirDownRight = Air;
pub type AirDownLeft = Air;

#[derive(Debug, Default, PartialEq)]
pub struct AirHold {
    /// The specific measure the note will be placed in.
    pub measure: usize,
    /// The offset of the note from the start of the specified measure.
    /// calculated with the method described in [`ChuniChart`][crate::ChuniChart]'s `resolution` field.
    pub offset: usize,
    /// The numerical ID for the column in the playfield that the note should
    /// appear in, ranging between 0-15, with 0 being the leftmost column, and
    /// 15 being the rightmost column.
    pub cell: usize,
    /// The width of the note, extending from the specified cell to the right.
    /// minimum value is 1, which means that the note only occupies the column
    /// specified.
    pub width: usize,
    pub target_note: String,
    pub duration: usize,
}

#[derive(Debug, Default, PartialEq)]
pub struct AirTrace {
    /// The specific measure the note will be placed in.
    pub measure: usize,
    /// The offset of the note from the start of the specified measure.
    /// calculated with the method described in [`ChuniChart`][crate::ChuniChart]'s `resolution` field.
    pub offset: usize,
    /// The numerical ID for the column in the playfield that the note should
    /// appear in, ranging between 0-15, with 0 being the leftmost column, and
    /// 15 being the rightmost column.
    pub cell: usize,
    /// The width of the note, extending from the specified cell to the right.
    /// minimum value is 1, which means that the note only occupies the column
    /// specified.
    pub width: usize,
    pub unknown: usize,
    pub starting_height: f64,
    pub duration: usize,
    pub end_cell: usize,
    pub end_width: usize,
    pub target_height: f64,
    pub color: String,
}

pub type AirCrush = AirTrace;

#[derive(Debug, Default, PartialEq)]
pub struct AirSlide {
    /// The specific measure the note will be placed in.
    pub measure: usize,
    /// The offset of the note from the start of the specified measure.
    /// calculated with the method described in [`ChuniChart`][crate::ChuniChart]'s `resolution` field.
    pub offset: usize,
    /// The numerical ID for the column in the playfield that the note should
    /// appear in, ranging between 0-15, with 0 being the leftmost column, and
    /// 15 being the rightmost column.
    pub cell: usize,
    /// The width of the note, extending from the specified cell to the right.
    /// minimum value is 1, which means that the note only occupies the column
    /// specified.
    pub width: usize,
    pub target_note: String,
    pub starting_height: f64,
    pub duration: usize,
    pub end_cell: usize,
    pub end_width: usize,
    pub target_height: f64,
    pub color: String,
}

pub type AirSlideControlPoint = AirSlide;

/// A mine note involves the player not touching the cell that the mine is
/// placed on. Touching the cell will result in the player losing score,
/// and possibly failing the track.
pub type Mine = Tap;

#[derive(Debug, PartialEq)]
pub enum NoteType {
    Tap(Tap),
    ExTap(ExTap),
    Hold(Hold),
    HoldWithExTapHead(HoldWithExTapHead),
    Slide(Slide),
    SlideControlPoint(SlideControlPoint),
    SlideWithExTapHead(SlideWithExTapHead),
    SlideControlPointWithExTapHead(SlideControlPointWithExTapHead),
    Flick(Flick),
    Air(Air),
    AirUpRight(AirUpRight),
    AirUpLeft(AirUpLeft),
    AirHold(AirHold),
    AirDown(AirDown),
    AirDownRight(AirDownRight),
    AirDownLeft(AirDownLeft),
    AirCrush(AirCrush),
    AirSlide(AirSlide),
    AirSlideControlPoint(AirSlideControlPoint),
    Mine(Mine),
}

impl FromStr for NoteType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (note_type, data) = s.trim().split_once('\t').unwrap();

        let mut data = data.trim().split('\t');

        let measure = data.next().unwrap().parse::<usize>().unwrap();
        let offset = data.next().unwrap().parse::<usize>().unwrap();
        let cell = data.next().unwrap().parse::<usize>().unwrap();
        let width = data.next().unwrap().parse::<usize>().unwrap();

        let note = match note_type {
            "TAP" => NoteType::Tap(Tap {
                measure,
                offset,
                cell,
                width,
            }),

            "CHR" => NoteType::ExTap(ExTap {
                measure,
                offset,
                cell,
                width,
                animation: data.next().unwrap().to_owned(),
            }),

            "HLD" => NoteType::Hold(Hold {
                measure,
                offset,
                cell,
                width,
                duration: data.next().unwrap().parse::<usize>().unwrap(),
                animation: data.next().map(|x| x.to_string()),
            }),

            "HXD" => NoteType::HoldWithExTapHead(HoldWithExTapHead {
                measure,
                offset,
                cell,
                width,
                duration: data.next().unwrap().parse::<usize>().unwrap(),
                animation: data.next().map(|x| x.to_string()),
            }),

            "SLD" => NoteType::Slide(Slide {
                measure,
                offset,
                cell,
                width,
                duration: data.next().unwrap().parse::<usize>().unwrap(),
                end_cell: data.next().unwrap().parse::<usize>().unwrap(),
                end_width: data.next().unwrap().parse::<usize>().unwrap(),
                animation: data.nth(1).map(|x| x.to_string()),
            }),

            "SLC" => NoteType::SlideControlPoint(SlideControlPoint {
                measure,
                offset,
                cell,
                width,
                duration: data.next().unwrap().parse::<usize>().unwrap(),
                end_cell: data.next().unwrap().parse::<usize>().unwrap(),
                end_width: data.next().unwrap().parse::<usize>().unwrap(),
                animation: data.nth(1).map(|x| x.to_string()),
            }),

            "SXD" => NoteType::SlideWithExTapHead(SlideWithExTapHead {
                measure,
                offset,
                cell,
                width,
                duration: data.next().unwrap().parse::<usize>().unwrap(),
                end_cell: data.next().unwrap().parse::<usize>().unwrap(),
                end_width: data.next().unwrap().parse::<usize>().unwrap(),
                animation: data.nth(1).map(|x| x.to_string()),
            }),

            "SXC" => NoteType::SlideControlPointWithExTapHead(SlideControlPointWithExTapHead {
                measure,
                offset,
                cell,
                width,
                duration: data.next().unwrap().parse::<usize>().unwrap(),
                end_cell: data.next().unwrap().parse::<usize>().unwrap(),
                end_width: data.next().unwrap().parse::<usize>().unwrap(),
                animation: data.nth(1).map(|x| x.to_string()),
            }),

            "FLK" => NoteType::Flick(Flick {
                measure,
                offset,
                cell,
                width,
                unknown: data.next().unwrap().to_string(),
            }),

            "AIR" => NoteType::Air(Air {
                measure,
                offset,
                cell,
                width,
                target_note: data.next().unwrap().to_string(),
            }),

            "AUL" => NoteType::AirUpLeft(AirUpLeft {
                measure,
                offset,
                cell,
                width,
                target_note: data.next().unwrap().to_string(),
            }),

            "AUR" => NoteType::AirUpRight(AirUpRight {
                measure,
                offset,
                cell,
                width,
                target_note: data.next().unwrap().to_string(),
            }),

            "AHD" => NoteType::AirHold(AirHold {
                measure,
                offset,
                cell,
                width,
                target_note: data.next().unwrap().to_string(),
                duration: data.next().unwrap().parse::<usize>().unwrap(),
            }),

            "ADW" => NoteType::AirDown(AirDown {
                measure,
                offset,
                cell,
                width,
                target_note: data.next().unwrap().to_string(),
            }),

            "ADL" => NoteType::AirDownLeft(AirDownLeft {
                measure,
                offset,
                cell,
                width,
                target_note: data.next().unwrap().to_string(),
            }),

            "ADR" => NoteType::AirDownRight(AirDownRight {
                measure,
                offset,
                cell,
                width,
                target_note: data.next().unwrap().to_string(),
            }),

            "ALD" => NoteType::AirCrush(AirCrush {
                measure,
                offset,
                cell,
                width,
                unknown: data.next().unwrap().parse::<usize>().unwrap(),
                starting_height: data.next().unwrap().parse::<f64>().unwrap(),
                duration: data.next().unwrap().parse::<usize>().unwrap(),
                end_cell: data.next().unwrap().parse::<usize>().unwrap(),
                end_width: data.next().unwrap().parse::<usize>().unwrap(),
                target_height: data.next().unwrap().parse::<f64>().unwrap(),
                color: if let Some(t) = data.next() {
                    match t {
                        "GRY" | "RED" | "ORN" | "YEL" | "AQA" | "PPL" | "PNK" | "CYN" | "BLK"
                        | "VLT" | "LIM" | "BLU" | "NON" | "DEF" => t.to_string(),
                        _ => unreachable!(
                            "invalid color found while parsing air trace / air crush. bailing."
                        ),
                    }
                } else {
                    unreachable!("cannot parse color for air trace / air crush. bailing.")
                },
            }),

            "ASD" => NoteType::AirSlide(AirSlide {
                measure,
                offset,
                cell,
                width,
                target_note: data.next().unwrap().to_string(),
                starting_height: data.next().unwrap().parse::<f64>().unwrap(),
                duration: data.next().unwrap().parse::<usize>().unwrap(),
                end_cell: data.next().unwrap().parse::<usize>().unwrap(),
                end_width: data.next().unwrap().parse::<usize>().unwrap(),
                target_height: data.next().unwrap().parse::<f64>().unwrap(),
                color: if let Some(t) = data.next() {
                    match t {
                        "GRY" | "RED" | "ORN" | "YEL" | "AQA" | "PPL" | "PNK" | "CYN" | "BLK"
                        | "VLT" | "LIM" | "BLU" | "NON" | "DEF" => t.to_string(),
                        _ => unreachable!(
                            "invalid color found while parsing air trace / air crush. bailing."
                        ),
                    }
                } else {
                    unreachable!("cannot parse color for air trace / air crush. bailing.")
                },
            }),

            "MNE" => NoteType::Mine(Mine {
                measure,
                offset,
                cell,
                width,
            }),

            _ => unreachable!("something broke. bailing."),
        };

        Ok(note)
    }
}
