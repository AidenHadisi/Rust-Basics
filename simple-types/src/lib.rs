pub fn print_difference(x: f32, y: f32) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

pub fn print_array(a: [f32; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

pub fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

pub fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
}

/// prints distance between x and y, which are stored in a tuple.
pub fn print_distance((x, y): (f32, f32)) {
    //Calculates and prints distance
    println!(
        "Distance to the origin is {}",
        (x.powf(2.0) + y.powf(2.0)).sqrt()
    );
}
