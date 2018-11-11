/*
cargo run -p traits --bin print_news2
*/
use std::fmt;

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

impl fmt::Display for Fact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

fn print_news<T: Truth + fmt::Display>(facts: &[T]) {
    for fact in facts {
        let fact = fact.make_true();
        println!("{}", fact);
    }
}

fn main() {
    let facts = vec![Fact {
        text: String::from("lorem ipsum"),
    }];

    print_news(&facts);

    //    let fact = Fact {
    //        text: String::from("Hello!"),
    //    };
    //    let i32_value = 0;
    //    let slice: &[Truth] = &[fact, i32_value];
    //
    //    print_news(slice);
}
