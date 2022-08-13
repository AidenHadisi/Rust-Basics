// Silence some warnings.
#![allow(dead_code, unused_variables)]

use simple_types::{ding, on_off, print_array, print_difference, print_distance};

const UNDERLINE: i32 = 100_000;
const BYTE: u8 = b'A';

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);

    print_difference(coords.0, coords.1);

    // create an array from tuple
    let coords_arr: [f32; 2] = [coords.0, coords.1];
    print_array(coords_arr);

    let series = [1, 1, 2, 3, 5, 8, 13];

    //passing the last value
    ding(series[series.len() - 1]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    //accessing deeply nested stuff
    on_off(mess.2[1].0);

    print_distance(coords);

    //HOW TO PASS LITERAL WITH TYPE
    let x = 5u16;
    //or even better
    let y = 5_u16;

    //casting
    let z = y as i32;
}
