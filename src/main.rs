mod blackjack;

fn main() {
    blackjack::play().unwrap_or_else(|err| print!("{}", err))
}
