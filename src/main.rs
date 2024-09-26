mod blackjack;
mod dealer;

fn main() {
    blackjack::play().unwrap_or_else(|err| print!("{}", err))
}
