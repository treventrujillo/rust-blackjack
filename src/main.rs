mod blackjack;
mod dealer;

fn main() {
    match blackjack::play() {
        Ok(_result) => print!("Round complete. Place your bets."),
        Err(err) => print!("{}", err)
    }
}
