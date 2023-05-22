mod engine;
mod flags;

use engine::Engine;
use std::{env, time::Instant};

/// wc - print the number of lines, words, and bytes in files
///
/// This is a re-implementation of the coreutils wc command, available at;
/// https://git.savannah.gnu.org/cgit/coreutils.git/tree/src/wc.c
///
fn main() {
    let now = Instant::now();

    let wc = Engine::new(env::args());

    wc.run();

    // Only print the time taken in debug mode
    if cfg!(debug_assertions) {
        let elapsed = now.elapsed();
        println!("Took: {}ms ({}ns)", elapsed.as_millis(), elapsed.as_nanos());
    }
}
