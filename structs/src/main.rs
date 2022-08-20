trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)]
struct Grapes {
    left: i32,
}

impl Bite for Grapes {
    fn bite(&mut self) {
        self.left -= 1;
    }
}

fn main() {
    let mut carrot = Carrot {
        percent_left: 100.0,
    };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}

fn bunny_nibbles<T: Bite>(animal: &mut T) {
    for _ in 1..5 {
        animal.bite()
    }
}

#[derive(Debug)]
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        self.percent_left *= 0.8;
    }
}
