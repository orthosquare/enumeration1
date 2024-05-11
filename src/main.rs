mod collections;
mod enumerators;

use std::time::Instant;

use enumerators::arraykt;

fn main() {
    for i in 2..9 {
        for j in 1..(i/2 + 1) {
            let now = Instant::now();
            let agg = arraykt::bin_square(i, j);

            println!("({}, {}):\t{}\t{}", i, j, agg, (now.elapsed().as_micros() as f64)/1000.0);
        }
    }
}
