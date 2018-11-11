/*
cargo run -p data_and_behavior --bin add_method
*/

#[derive(Debug, PartialEq)]
struct Fact {
    text: String,
}

impl Fact {
    fn make_true(&self) -> Fact {
        Fact {
            text: format!("{}!!", self.text),
        }
    }
}

fn main() {
    let fact = Fact {
        text: String::from("Lorem ipsum"),
    };
    let true_fact = fact.make_true();
    println!("{}", true_fact.text);
    assert_ne!(fact, true_fact);
}
