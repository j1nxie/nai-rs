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
    pub end_cell: usize,
    pub end_width: usize,
    pub target_height: f64,
    pub color: String,
}

pub type Mine = Tap;

#[derive(Debug, PartialEq)]
pub enum NoteType {
    Tap(Tap),
    ExTap(ExTap),
    Hold(Hold),
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
    Mine(Mine),
}
