#[derive(Debug, Default, PartialEq)]
pub struct Tap {
    pub measure: usize,
    pub offset: usize,
    pub cell: usize,
    pub width: usize,
}

#[derive(Debug, Default, PartialEq)]
pub struct ExTap {
    pub measure: usize,
    pub offset: usize,
    pub cell: usize,
    pub width: usize,
    pub animation: String,
}

#[derive(Debug, Default, PartialEq)]
pub struct Hold {
    pub measure: usize,
    pub offset: usize,
    pub cell: usize,
    pub width: usize,
    pub duration: usize,
    pub animation: Option<String>,
}

pub type HoldWithExTapHead = Hold;

#[derive(Debug, Default, PartialEq)]
pub struct Slide {
    pub measure: usize,
    pub offset: usize,
    pub cell: usize,
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
    pub measure: usize,
    pub offset: usize,
    pub cell: usize,
    pub width: usize,
    pub unknown: String,
}

#[derive(Debug, Default, PartialEq)]
pub struct Air {
    pub measure: usize,
    pub offset: usize,
    pub cell: usize,
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
    pub measure: usize,
    pub offset: usize,
    pub cell: usize,
    pub width: usize,
    pub target_note: String,
    pub duration: usize,
}

#[derive(Debug, Default, PartialEq)]
pub struct AirTrace {
    pub measure: usize,
    pub offset: usize,
    pub cell: usize,
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
    pub measure: usize,
    pub offset: usize,
    pub cell: usize,
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

pub fn parse_note(line: &str) -> NoteType {
    let (note_type, data) = line.trim().split_once('\t').unwrap();

    let mut data = data.trim().split('\t');

    let measure = data.next().unwrap().parse::<usize>().unwrap();
    let offset = data.next().unwrap().parse::<usize>().unwrap();
    let cell = data.next().unwrap().parse::<usize>().unwrap();
    let width = data.next().unwrap().parse::<usize>().unwrap();

    match note_type {
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
                    | "YEL" | "VLT" | "LIM" | "BLU" | "NON" | "DEF" => t.to_string(),
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
                    | "YEL" | "VLT" | "LIM" | "BLU" | "NON" | "DEF" => t.to_string(),
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
    }
}
