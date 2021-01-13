extern crate pretty_env_logger;

extern crate twisted;

use std::io::{self, Write};

use twisted::coord::Coord;
use twisted::cube::cube3::{
    coord::{
        CornerOrientCoord, CornerPosCoord, EEdgePosCoord, ESliceAndEOCoord, ESliceEdgePosCoord,
        EdgeOrientCoord, Phase1Coord, Phase2Coord, Phase2MinusECoord, UdEdgePosCoord,
    },
    notation::Cube3Notation,
    Cube3Perm, CubeTurn, G1CubeTurn,
};
use twisted::move_table::{BasicMoveTable, CompositeMoveTable, MoveTable};
use twisted::notation::{NotationMove, NotationStr};
use twisted::prune_table::{CompositePruneTable, FullPruneTable, PruneTable, ZeroPruneTable};
use twisted::puzzle::PuzzlePerm;
use twisted::solver::{solve_cube, SolutionIter};

type Notation = NotationStr<Cube3Notation>;

fn do_phase_solve<C, M, P>(
    move_table: &M,
    prune_table: &P,
    perm: &Cube3Perm,
    target: C,
) -> Option<Notation>
where
    C: Coord<Cube3Perm>,
    M: MoveTable<Puzzle = Cube3Perm, Coord = C>,
    P: PruneTable<Puzzle = Cube3Perm, Coord = C, Move = M::Move>,
    M::Move: Into<NotationMove<Cube3Notation>>,
{
    SolutionIter::new(move_table, prune_table, target, perm)
        .next()
        .map(|sol| {
            Notation::from(
                sol.iter()
                    .map(|m| (*m).into())
                    .collect::<Vec<NotationMove<Cube3Notation>>>(),
            )
        })
}

fn do_solve<M1, P1, M2, P2>(
    phase1_move_table: &M1,
    phase1_prune_table: &P1,
    phase2_move_table: &M2,
    phase2_prune_table: &P2,
    notation: Notation,
) where
    M1: MoveTable<Puzzle = Cube3Perm, Coord = Phase1Coord, Move = CubeTurn>,
    P1: PruneTable<Puzzle = Cube3Perm, Coord = Phase1Coord, Move = CubeTurn>,
    M2: MoveTable<Puzzle = Cube3Perm, Coord = Phase2Coord, Move = G1CubeTurn>,
    P2: PruneTable<Puzzle = Cube3Perm, Coord = Phase2Coord, Move = G1CubeTurn>,
{
    let perm = notation.permutation();

    let phase1_sol_generator = SolutionIter::new(
        phase1_move_table,
        phase1_prune_table,
        Phase1Coord::default(),
        &perm,
    );
    for phase1_sol_moves in phase1_sol_generator.take(5) {
        let phase1_solution = Notation::from(
            phase1_sol_moves
                .iter()
                .map(|m| (*m).into())
                .collect::<Vec<NotationMove<Cube3Notation>>>(),
        );

        // println!("Phase 1 solution: {}", phase1_solution);

        // Check it
        let phase1_perm = perm.sequence(&phase1_solution.permutation());
        let g1_coord = Phase1Coord::from_perm(&phase1_perm);
        if g1_coord != Phase1Coord::default() {
            eprintln!("Error, invalid solution (Coord = {:?})", g1_coord);
        }

        let (phase2_solution, solved_perm) = match do_phase_solve(
            phase2_move_table,
            phase2_prune_table,
            &phase1_perm,
            Phase2Coord::default(),
        ) {
            Some(sol) => {
                // println!("Phase 2 solution: {}", sol);

                // Check it
                let solved = phase1_perm.sequence(&sol.permutation());
                let solved_coord = Phase2Coord::from_perm(&solved);
                if solved_coord != Phase2Coord::default() {
                    eprintln!("Error, invalid solution (Coord = {:?})", solved_coord);
                }

                (sol, solved)
            }
            None => {
                println!("No phase 2 solution found");
                return;
            }
        };

        let l1 = phase1_solution.len();
        let l2 = phase2_solution.len();
        println!(
            "Solution ({} + {} = {}): {} {}",
            l1,
            l2,
            l1 + l2,
            phase1_solution,
            phase2_solution
        );
    }

    println!();

    /*
    let (phase1_solution, phase1_perm) = match do_phase_solve(
        phase1_move_table,
        phase1_prune_table,
        &perm,
        Phase1Coord::default(),
    ) {
        Some(sol) => {
            println!("Phase 1 solution: {}", sol);

            // Check it
            let solved = perm.sequence(&sol.permutation());
            let solved_coord = Phase1Coord::from_perm(&solved);
            if solved_coord != Phase1Coord::default() {
                println!("Error, invalid solution (Coord = {:?})", solved_coord);
            }

            (sol, solved)
        }
        None => {
            println!("No phase 1 solution found");
            return;
        }
    };

    let (phase2_solution, solved_perm) = match do_phase_solve(
        phase2_move_table,
        phase2_prune_table,
        &phase1_perm,
        Phase2Coord::default(),
    ) {
        Some(sol) => {
            println!("Phase 2 solution: {}", sol);

            // Check it
            let solved = phase1_perm.sequence(&sol.permutation());
            let solved_coord = Phase2Coord::from_perm(&solved);
            if solved_coord != Phase2Coord::default() {
                println!("Error, invalid solution (Coord = {:?})", solved_coord);
            }

            (sol, solved)
        }
        None => {
            println!("No phase 2 solution found");
            return;
        }
    };

    println!("Solution: {} {}", phase1_solution, phase2_solution);
    */
}

fn main() {
    pretty_env_logger::init();

    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut input_buf = String::new();

    println!("Initializing tables...");

    // Phase 1

    println!("Corner orient...");
    let co_table: BasicMoveTable<Cube3Perm, CornerOrientCoord, CubeTurn> = BasicMoveTable::create();

    println!("Edge orient...");
    let eo_table: BasicMoveTable<Cube3Perm, EdgeOrientCoord, CubeTurn> = BasicMoveTable::create();

    println!("E edge location...");
    let phase1_eslice_table: BasicMoveTable<Cube3Perm, EEdgePosCoord, CubeTurn> =
        BasicMoveTable::create();

    println!("Edge orient and E edge table...");
    let phase1_edge_table = CompositeMoveTable::new(&phase1_eslice_table, &eo_table).to_basic();

    let phase1_move_table = CompositeMoveTable::new(&co_table, &phase1_edge_table);

    println!("Corner orient prune...");
    let co_prune_table = FullPruneTable::create(&co_table, CornerOrientCoord::default());
    println!("Edge orient prune...");
    let phase1_edge_prune_table =
        FullPruneTable::create(&phase1_edge_table, ESliceAndEOCoord::default());

    let phase1_prune_table = CompositePruneTable::new(&co_prune_table, &phase1_edge_prune_table);

    // println!("Phase 1 prune...");
    // let phase1_prune_table = FullPruneTable::create(&phase1_move_table, Phase1Coord::default());

    // Phase 2

    println!("Corner permutation...");
    let cp_table = BasicMoveTable::create();

    println!("UD edge permutation...");
    let ud_ep_table = BasicMoveTable::create();

    println!("E Edge permutation...");
    let phase2_eslice_table = BasicMoveTable::create();

    let phase2_minus_e_table = CompositeMoveTable::new(&cp_table, &ud_ep_table);
    let phase2_move_table = CompositeMoveTable::new(&phase2_minus_e_table, &phase2_eslice_table);

    println!("Phase2 prune...");
    // Pruning table only uses corner permutation and UD edge permutation
    let cp_prune_table = FullPruneTable::create(&cp_table, CornerPosCoord::default());
    let ud_ep_prune_table = FullPruneTable::create(&ud_ep_table, UdEdgePosCoord::default());
    let phase2_minus_e_prune_table = CompositePruneTable::new(&cp_prune_table, &ud_ep_prune_table);
    // let phase2_minus_e_prune_table =
    // FullPruneTable::create(&phase2_minus_e_table, Phase2MinusECoord::default());
    let e_slice_prune_table =
        FullPruneTable::create(&phase2_eslice_table, ESliceEdgePosCoord::default());
    let phase2_prune_table =
        CompositePruneTable::new(&phase2_minus_e_prune_table, &e_slice_prune_table);

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
                    Ok(notation) => do_solve(
                        &phase1_move_table,
                        &phase1_prune_table,
                        &phase2_move_table,
                        &phase2_prune_table,
                        notation,
                    ),
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
