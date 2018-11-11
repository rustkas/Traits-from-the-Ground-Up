/*
cargo run -p traits --bin trait
*/

trait Truth {
    fn make_true(&self) -> Self;
}

#[derive(Debug, PartialEq)]
struct Fact {
    text: String,
}
impl Truth for Fact {
    fn make_true(&self) -> Self {
        Fact {
            text: format!("{}!!", self.text),
        }
    }
}
impl Truth for i32 {
    fn make_true(&self) -> Self {
        42
    }
}

fn main() {
    let fact = Fact {
        text: String::from("Hello!"),
    };
    let new_fact = fact.make_true();

    assert_ne!(fact, new_fact);
    println!("{}", new_fact.text);

    let i32_value = 0;
    assert_eq!(42, i32_value.make_true());
    println!("new i32 value is {}", i32_value.make_true());
}
