/*
cargo run -p data_and_behavior --bin unambiguous_function_call
*/
trait Truth {
    fn make_true(&self) -> Self;
}

trait FakeTruth {
    fn make_true(&self) -> Self;
}

struct Fact {
    text: String,
}
impl Truth for Fact {
    fn make_true(&self) -> Self {
        Fact {
            text: format!("{} This is a truth", self.text),
        }
    }
}

impl FakeTruth for Fact {
    fn make_true(&self) -> Self {
        Fact {
            text: format!("{} This is a fake truth", self.text),
        }
    }
}

fn main(){
    let fact = Fact {
        text: String::from("No facts."),
    };
    let truth = <Fact as Truth>::make_true(&fact);
    let fake_truth = <Fact as FakeTruth>::make_true(&fact);
    println!("{}", truth.text);
    println!("{}", fake_truth.text);
}
