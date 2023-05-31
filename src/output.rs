#[derive(Clone, Debug)]
pub struct X {
    pub test1: String,
    pub test2: String,
}

impl Newstruct {
    pub fn new(test1: String, test2: String) -> X {
        X { test1, test2 }
    }
}

fn main() {
    let mut x: X = X::new("123".to_string(), "456".to_string());
    //= y { | x = test1 "1" }
    x.test1 = "1".to_string();
    println!("{:?}", &x.clone());
}
