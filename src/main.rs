use enum_based::GameState;

mod enum_based;

fn main() {
    let state = GameState::new_with_default_setup();

    println!("{state}");
    // state.do_move([0, 6]);
    // println!("{state}");
    // state.do_move([5, 0]);
    // println!("{state}");
    // state.do_move([99, 0]);
    // println!("{state}");
    // state.do_move([5, 99]);
    // println!("{state}");

    // let t = Instant::now();
    // let e = state.get_brute_force_eval(false, 3);
    // let t = t.elapsed();
    // println!("{e}");
    // println!("{t:?}");

    // let mut moves = Vec::new();

    // state.get_possible_moves_double(
    //     false,
    //     [5, 6],
    //     &mut moves,
    //     &mut Vec::new(),
    //     &mut Vec::new(),
    // );
    // for m in moves {
    //     println!("{m:2?}");
    //     let mut new_state = state.clone();
    //     for m in m {
    //         new_state.do_move(m);
    //     }
    //     println!("{new_state}");
    // }
}
