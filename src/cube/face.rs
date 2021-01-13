//! Face representation of a 2x2x2 cube puzzle

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Face {
    Up,
    Right,
    Front,
    Down,
    Left,
    Back,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaceColor {
    White,
    Red,
    Green,
    Yellow,
    Orange,
    Blue,
}

impl From<Face> for FaceColor {
    fn from(face: Face) -> Self {
        match face {
            Face::Up => Self::White,
            Face::Right => Self::Red,
            Face::Front => Self::Green,
            Face::Down => Self::Yellow,
            Face::Left => Self::Orange,
            Face::Back => Self::Blue,
        }
    }
}
