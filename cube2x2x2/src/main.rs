#[macro_use]
extern crate lazy_static;

extern crate twisted;

use std::io::{self, Write};

use twisted::cube::cube2::{
    coord::{Corner7Coord, CornerOrient7Coord, CornerPos7Coord},
    render::print_cube,
    symmetry::fix_dbl_corner,
    Cube2Notation, UrfTurn,
    Cube2Perm,
};
use twisted::move_table::{BasicMoveTable, CompositeMoveTable};
use twisted::notation::{NotationMove, NotationStr};
use twisted::prune_table::FullPruneTable;
use twisted::solver::solve_cube;

type Notation = NotationStr<Cube2Notation>;

type FinalMoveTable = CompositeMoveTable<
    'static,
    Corner7Coord,
    BasicMoveTable<Cube2Perm, CornerOrient7Coord, UrfTurn>,
    BasicMoveTable<Cube2Perm, CornerPos7Coord, UrfTurn>,
>;

lazy_static! {
    static ref ORIENT_TABLE: BasicMoveTable<Cube2Perm, CornerOrient7Coord, UrfTurn> = BasicMoveTable::create();
    static ref POS_TABLE: BasicMoveTable<Cube2Perm, CornerPos7Coord, UrfTurn> = BasicMoveTable::create();
    static ref MOVE_TABLE: FinalMoveTable = CompositeMoveTable::new(&*ORIENT_TABLE, &*POS_TABLE);
    static ref PRUNE_TABLE: FullPruneTable<Corner7Coord, UrfTurn> =
        FullPruneTable::create(&*MOVE_TABLE, Corner7Coord::default());
}

fn do_solve(notation: Notation) {
    let perm = notation.permutation();
    print_cube(&perm);

    let (_sym, perm) = fix_dbl_corner(&perm);
    match solve_cube(&*MOVE_TABLE, &*PRUNE_TABLE, &perm, Corner7Coord::default()) {
        Some(sol) => {
            let sol_notation = Notation::from(
                sol.iter()
                    .map(|m| NotationMove::<Cube2Notation>::from(*m))
                    .collect::<Vec<_>>(),
            );
            println!("Solution: {}", sol_notation);
        }
        None => {
            println!("No solution found");
        }
    }
}

fn main() {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut input_buf = String::new();

    println!("Initializing tables...");
    lazy_static::initialize(&ORIENT_TABLE);
    println!("Orient");
    lazy_static::initialize(&POS_TABLE);
    println!("Pos");
    lazy_static::initialize(&MOVE_TABLE);
    println!("Move");
    lazy_static::initialize(&PRUNE_TABLE);
    println!("Prune");
    println!("Done");

    loop {
        input_buf.clear();

        print!("Scramble: ");
        stdout.flush().expect("Error flushing stream");

        match stdin.read_line(&mut input_buf) {
            Ok(_) => {
                if input_buf.is_empty() {
                    break;
                }

                match input_buf.parse() {
                    Ok(notation) => do_solve(notation),
                    Err(_) => {
                        println!("Invalid cube notation");
                    }
                }
            }
            Err(err) => {
                eprint!("{}", err);
                break;
            }
        }
    }
}
