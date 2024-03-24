const MAX_PLAYERS: u8 = 10;
static CASINO_NAME: &str = "The Rusty Casino";

fn main() {
    let a: i32 = 10;
    let b: i32 = 10;

    let c: &str = CASINO_NAME;
    let d: &str = CASINO_NAME;
    
    println!("Let's play a game of poker at {CASINO_NAME}!");
    println!("MAX_PLAYERS: {MAX_PLAYERS}");
    println!("a: {a}, b: {b}, c: {c}, d: {d}");
}
