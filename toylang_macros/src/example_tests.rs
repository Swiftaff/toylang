pub struct ExampleTests {
    pub tests: Vec<(String, String, String)>,
}

impl ExampleTests {
    pub fn new() -> ExampleTests {
        let test_strs = vec![
            ("test_pass_empty_file", "", "fn main() {\r\n}\r\n"),
            ("test2", "c", "d"),
            ("test3", "e", "f"),
            ("test4", "g", "h"),
        ];
        let tests = test_strs
            .iter()
            .map(|(a, b, c)| (a.to_string(), b.to_string(), c.to_string()))
            .collect::<Vec<(String, String, String)>>();
        ExampleTests { tests }
    }
}
