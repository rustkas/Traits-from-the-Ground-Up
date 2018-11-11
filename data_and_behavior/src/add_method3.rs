/*
cargo run -p data_and_behavior --bin add_method3
*/

struct Fact {
    text: String,
}

impl Fact {
    fn make_true(mut self) -> Fact {
        self.text.push_str("!!");
        self
    }
}
fn main() {
    let fact = Fact {
        text: String::from("Lorem ipsum"),
    };
    println!("{}", fact.text);

    let true_fact = fact.make_true();
    println!("{}", true_fact.text);
}
