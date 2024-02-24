#[derive(Debug, Default, PartialEq)]
pub struct Tap {
    pub measure: usize,
    pub offset: usize,
    pub cell: usize,
    pub width: usize,
}

#[derive(Debug, Default, PartialEq)]
pub struct ExNote {
    pub measure: usize,
    pub offset: usize,
    pub cell: usize,
    pub width: usize,
    pub unknown: String,
}

#[derive(Debug, Default, PartialEq)]
pub struct Hold {
    pub measure: usize,
    pub offset: usize,
    pub cell: usize,
    pub width: usize,
    pub duration: usize,
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
}

pub type SlideControlPoint = Slide;

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
    pub target_note: usize,
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
    pub target_note: usize,
    pub duration: usize,
}

pub type Mine = Tap;

#[derive(Debug, PartialEq)]
pub enum NoteType {
    Tap(Tap),
    ExNote(ExNote),
    Hold(Hold),
    Slide(Slide),
    SlideControlPoint(SlideControlPoint),
    Flick(Flick),
    Air(Air),
    AirUpRight(AirUpRight),
    AirUpLeft(AirUpLeft),
    AirHold(AirHold),
    AirDown(AirDown),
    AirDownRight(AirDownRight),
    AirDownLeft(AirDownLeft),
    Mine(Mine),
}
