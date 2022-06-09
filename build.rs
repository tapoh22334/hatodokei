// build.rs

extern crate windres;

use windres::Build;

fn main() {
    Build::new().compile("hatodokei.rc").unwrap();
}
