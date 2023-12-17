use crate::{engine::term, game::cells, Params};
use std::{thread, time};

/// Runs the simulation and the render,
/// being a window or in terminal based on [params](Params).
pub async fn run(params: Params) {
    let mut cells = cells::init_cells(params.size);

    if params.term {
        ctrlc::set_handler(move || {
            term::term_restore();
            std::process::exit(0);
        }).expect("Could not set ctrl+c handler");

        term::term_init();
    }
    // TODO: Separate render thread

    let frame_time = time::Duration::from_secs_f32(1.0 / params.tick);
    // Game loops until window is closed
    loop {
        let tick_time = time::Instant::now();

        if params.term {
            let _ = term::draw(&cells);
        }
        sim_tick(&mut cells).await;
        let tick_time = tick_time - time::Instant::now();

        // Throttle tick
        thread::sleep(frame_time - tick_time);
    }
}

/// Computes the next step of the simulation
async fn sim_tick(cells: &mut [Vec<cells::Cell>]) {
    for i in 0..(cells.len()) {
        for j in 0..(cells[0].len()) {
            cells::Cell::attack_neighbours(cells, (i, j)).await;
        }
    }
}
