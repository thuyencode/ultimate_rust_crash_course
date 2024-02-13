const STARTING_MISSILES: i32 = 8; 
const READY_AMOUNT: i32 = 2;

fn main() {
    part_1();
    println!();
    
    part_2();
    println!();

    extra();
}

fn part_1() {
    let missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;

    println!("Firing {} of my {} missiles...", ready, missiles);
}

fn part_2() {
    let mut missiles = 8;
    let ready = 2;

    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles -= ready;

    println!("{} missiles left", missiles);
}

fn extra() {
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);

    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);
}