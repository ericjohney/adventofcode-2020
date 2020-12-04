pub mod file_utils {
    extern crate regex;
    use regex::Regex;
    use std::fs;

    pub fn read_to_string(filename: &'static str) -> String {
        fs::read_to_string(filename).unwrap()
    }

    pub fn lines(filename: &'static str) -> Vec<String> {
        fs::read_to_string(filename)
            .unwrap()
            .lines()
            .map(|l| l.into())
            .collect()
    }

    pub fn parse_lines(filename: &'static str, expression: &str) -> Vec<Vec<String>> {
        let text = fs::read_to_string(filename).unwrap();
        Regex::new(expression)
            .unwrap()
            .captures_iter(&text)
            .map(|c| {
                c.iter()
                    .map(|i| i.unwrap().as_str().to_string().into())
                    .collect()
            })
            .collect()
    }
}
