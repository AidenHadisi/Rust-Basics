const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;

    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;

    println!("{} missiles left", missiles);

    //tuple way of assigning, it's like destructuring array
    let (mut reserved_missiles, reserved_ready) = (10, 5);
    println!(
        "Firing {} of my reserved {} missiles...",
        reserved_ready, reserved_missiles
    );

    reserved_missiles = reserved_missiles - reserved_ready;
    println!("{} reserved missiles left", reserved_missiles);
}
