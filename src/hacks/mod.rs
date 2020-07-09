use crate::config::Config;
use crate::sdk::FortniteContext;
use crate::CHEAT_TICKRATE;

use memlib::util::LoopTimer;
use std::error::Error;

// The main loop of the cheat
// Returns an error if there is an error with any of the tick functions
pub fn hack_loop(ctx: FortniteContext) -> Result<(), Box<dyn Error>> {
    let config = Config::default();

    let mut timer = LoopTimer::new(CHEAT_TICKRATE);

    loop {
        timer.wait();
    }

    Ok(())
}
