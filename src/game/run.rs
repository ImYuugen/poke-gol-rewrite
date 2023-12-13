use crate::{engine::term, game::cells, Params};
use std::{thread, time};

/// Runs the simulation and the render,
/// being a window or in terminal based on [params](Params).
pub fn run(params: Params) {
    let mut cells = cells::init_cells(params.size);

    // TODO: Separate render thread

    let frame_time = time::Duration::from_secs_f32(1.0 / params.tick);
    // Game loops until window is closed
    loop {
        let tick_time = time::Instant::now();

        sim_tick(&mut cells);
        if params.term {
            term::draw(&cells);
        }
        let tick_time = tick_time - time::Instant::now();

        // Throttle tick
        thread::sleep(frame_time - tick_time);
    }
}

/// Computes the next step of the simulation
fn sim_tick(_cells: &mut [Vec<cells::Cell>]) {}
