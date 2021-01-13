//! Terminal rendering for a 2x2x2 cube puzzle

use termion::color;

use crate::cube::corner::{Corner, CornerOrient, CornerPerm, CornerPos};

use super::super::face::{Face, FaceColor};

fn print_term_color(c: impl color::Color) {
    print!("{}  ", color::Bg(c));
}

fn print_face_color(face: Face) {
    use FaceColor::*;
    match face.into() {
        White => print_term_color(color::White),
        Red => print_term_color(color::Red),
        Green => print_term_color(color::Green),
        Yellow => print_term_color(color::Yellow),
        Orange => print_term_color(color::Magenta),
        Blue => print_term_color(color::Blue),
    }
}

fn print_facelet(cube: &CornerPerm, pos: CornerPos, orient: CornerOrient) {
    print_face_color(cube.get_face(Corner::new(pos, orient)));
}

fn print_space() {
    print!("{} ", color::Bg(color::Reset));
}

fn println_reset() {
    println!("{}", color::Bg(color::Reset));
}

pub fn print_cube(cube: &CornerPerm) {
    use CornerOrient::*;
    use CornerPos::*;

    println!();

    // Up
    print!("     ");
    print_facelet(cube, ULB, Oriented);
    print_facelet(cube, UBR, Oriented);
    println_reset();
    print!("     ");
    print_facelet(cube, UFL, Oriented);
    print_facelet(cube, URF, Oriented);

    println_reset();
    println!();

    // Left/Up
    print_facelet(cube, ULB, Clockwise);
    print_facelet(cube, UFL, AntiClockwise);

    // Front/Up
    print_space();
    print_facelet(cube, UFL, Clockwise);
    print_facelet(cube, URF, AntiClockwise);

    // Right/Up
    print_space();
    print_facelet(cube, URF, Clockwise);
    print_facelet(cube, UBR, AntiClockwise);

    // Back/Up
    print_space();
    print_facelet(cube, UBR, Clockwise);
    print_facelet(cube, ULB, AntiClockwise);

    println_reset();

    // Left/Down
    print_facelet(cube, DBL, AntiClockwise);
    print_facelet(cube, DLF, Clockwise);

    // Front/Down
    print_space();
    print_facelet(cube, DLF, AntiClockwise);
    print_facelet(cube, DFR, Clockwise);

    // Right/Down
    print_space();
    print_facelet(cube, DFR, AntiClockwise);
    print_facelet(cube, DRB, Clockwise);

    // Back/Down
    print_space();
    print_facelet(cube, DRB, AntiClockwise);
    print_facelet(cube, DBL, Clockwise);

    println_reset();
    println!();

    // Down
    print!("     ");
    print_facelet(cube, DLF, Oriented);
    print_facelet(cube, DFR, Oriented);
    println_reset();
    print!("     ");
    print_facelet(cube, DBL, Oriented);
    print_facelet(cube, DRB, Oriented);

    println_reset();
}
