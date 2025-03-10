#![allow(warnings)]
use gtest::{Program, System};
use pebbles_game::*;
use pebbles_game_io::*;

const USERS: u64 = 345;

fn init_game(sys: &System, total: u32, turn_max: u32) {
    sys.init_logger();

    let game = Program::current_opt(sys);
    sys.mint_to(USERS, 1000000000000000000);
    let res = game.send(
        USERS,
        PebblesInit {
            pebbles_count: total,
            max_pebbles_per_turn: turn_max,
            difficulty: DifficultyLevel::Easy,
        },
    );
}

#[test]
fn init_successed() {
    let sys = System::new();
    sys.init_logger();

    let game = Program::current_opt(&sys);
    sys.mint_to(USERS, 1000000000000000000);
    let res = game.send(
        USERS,
        PebblesInit {
            pebbles_count: 10,
            max_pebbles_per_turn: 9,
            difficulty: DifficultyLevel::Easy,
        },
    );
}

#[test]
fn init_failed() {
    let sys = System::new();
    sys.init_logger();

    let game = Program::current_opt(&sys);
    sys.mint_to(USERS, 1000000000000000000);
    let res = game.send(
        USERS,
        PebblesInit {
            pebbles_count: 10,
            max_pebbles_per_turn: 11,
            difficulty: DifficultyLevel::Easy,
        },
    );
}

#[test]
fn user_move() {
    let sys = System::new();
    init_game(&sys, 101, 3);
    let game = sys.get_program(1).unwrap();
    let gmstate: PebbleGame = game.read_state(0).expect("Invalid state.");
    let mut remaing = gmstate.pebbles_remaining;

    let res = game.send(USERS, PebblesAction::Turn(1));
    sys.run_next_block();
    let gmstate: PebbleGame = game.read_state(0).expect("Invalid state.");

    assert_eq!(gmstate.pebbles_remaining, 98);

    remaing = gmstate.pebbles_remaining;
    let res = game.send(USERS, PebblesAction::Turn(2));
    sys.run_next_block();
    let gmstate: PebbleGame = game.read_state(0).expect("Invalid state.");

    assert_eq!(
        gmstate.pebbles_remaining,
        remaing - 2 - gmstate.program_lastmove
    );

    // User makes another move
    remaing = gmstate.pebbles_remaining;
    let res = game.send(USERS, PebblesAction::Turn(3));
    sys.run_next_block();
    let gmstate: PebbleGame = game.read_state(0).expect("Invalid state.");

    assert_eq!(
        gmstate.pebbles_remaining,
        remaing - 3 - gmstate.program_lastmove
    );
}

#[test]
fn user_move_failed() {
    let sys = System::new();
    init_game(&sys, 5, 2);
    let game = sys.get_program(1).unwrap();

    let res = game.send(USERS, PebblesAction::Turn(0));

    let res = game.send(USERS, PebblesAction::Turn(3));
}

#[test]
fn user_move_failed2() {
    let sys2 = System::new();
    init_game(&sys2, 3, 2);

    let game = sys2.get_program(1).unwrap();
    loop {
        let gmstate: PebbleGame = game.read_state(0).expect("Invalid state.");
        if gmstate.program_lastmove == 2 {
            break;
        }
        game.send(
            USERS,
            PebblesAction::Restart {
                difficulty: DifficultyLevel::Easy,
                pebbles_count: 3,
                max_pebbles_per_turn: 2,
            },
        );
        sys2.run_next_block();
    }
    let res = game.send(USERS, PebblesAction::Turn(2));
}

#[test]
fn program_move() {
    let sys = System::new();
    init_game(&sys, 99, 3);
    let game = sys.get_program(1).unwrap();
    let gmstate: PebbleGame = game.read_state(0).expect("Invalid state.");
    let mut remaing = gmstate.pebbles_remaining;

    let res = game.send(USERS, PebblesAction::GiveUp);
    sys.run_next_block();
    let gmstate: PebbleGame = game.read_state(0).expect("Invalid state.");

    assert_eq!(gmstate.pebbles_remaining, 97);

    remaing = gmstate.pebbles_remaining;
    let res = game.send(USERS, PebblesAction::GiveUp);
    sys.run_next_block();
    let gmstate: PebbleGame = game.read_state(0).expect("Invalid state.");

    assert_eq!(
        gmstate.pebbles_remaining,
        remaing - gmstate.program_lastmove
    );

    remaing = gmstate.pebbles_remaining;
    let res = game.send(USERS, PebblesAction::GiveUp);
    sys.run_next_block();
    let gmstate: PebbleGame = game.read_state(0).expect("Invalid state.");

    assert_eq!(
        gmstate.pebbles_remaining,
        remaing - gmstate.program_lastmove
    );
}

#[test]
fn winner() {
    let sys = System::new();
    init_game(&sys, 3, 1);
    let game = sys.get_program(1).unwrap();

    for _ in 0..100 {
        game.send(
            USERS,
            PebblesAction::Restart {
                difficulty: DifficultyLevel::Easy,
                pebbles_count: 3,
                max_pebbles_per_turn: 1,
            },
        );
        sys.run_next_block();
        let gmstate: PebbleGame = game.read_state(0).expect("Invalid state.");
        let remaing = gmstate.pebbles_remaining;
        if remaing < 3 {
            let res = game.send(USERS, PebblesAction::Turn(1));
        } else {
            let res = game.send(USERS, PebblesAction::Turn(1));
            let res = game.send(USERS, PebblesAction::Turn(1));
        }
    }
}

#[test]
fn restart() {
    let sys = System::new();
    init_game(&sys, 3, 1);
    let game = sys.get_program(1).unwrap();
    let res = game.send(
        USERS,
        PebblesAction::Restart {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 50,
            max_pebbles_per_turn: 3,
        },
    );
    sys.run_next_block();
    let gmstate: PebbleGame = game.read_state(0).expect("Invalid state.");
    assert_eq!(gmstate.pebbles_count, 50);
    assert_eq!(gmstate.max_pebbles_per_turn, 3);
}
