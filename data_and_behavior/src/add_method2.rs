/*
cargo run -p data_and_behavior --bin add_method2
*/

struct Fact {
    text: String,
}

impl Fact {
    fn make_true(&mut self) {
        self.text.push_str("!!");
    }
}

fn main() {
    let mut fact = Fact {
        text: String::from("Lorem ipsum"),
    };
    println!("{}", fact.text);
    fact.make_true();
    println!("{}", fact.text);
}
