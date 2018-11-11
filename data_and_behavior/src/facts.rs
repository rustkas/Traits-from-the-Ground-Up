/*
cargo run -p data_and_behavior --bin facts
*/
struct Fact {
    text: String,
}

fn make_true(input: &Fact) -> Fact {
    Fact {
        text: format!("{}!!", input.text),
    }
}

fn main() {
    let mut fact = Fact {
        text: String::from("Hello, Pascal"),
    };
    fact = make_true(&fact);
    println!("{}", fact.text);
}
