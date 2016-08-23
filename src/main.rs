extern crate rand;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;
#[macro_use]
extern crate enum_primitive;
extern crate num;
extern crate itertools;

mod cell;
mod fluid;
mod grid;

fn main() {
    let mut g = grid::Grid::new(100, 180);
    g.cycle();
}
