use crate::state::ServerState;
use std::convert::Infallible;
use std::time::{Duration, Instant};

pub fn server_game_loop(
    state: ServerState,
    update: fn(&mut ServerState, &Duration),
    fixed_update: fn(&mut ServerState, &Duration),
    interval: Duration,
) -> Infallible {
    let mut last_tick = Instant::now();
    let mut last_fixed_tick = Instant::now();
    let mut state = state;
    loop {
        let now = Instant::now();

        update(&mut state, &(last_tick - now));
        last_tick = now;

        if last_fixed_tick.elapsed() < interval {
            continue;
        }

        fixed_update(&mut state, &(last_fixed_tick - now));
        last_fixed_tick = now;
    }
}
