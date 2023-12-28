use std;

pub struct InputManager;

pub enum Prompt {
    Text(String),
    Number(i32),
    Quit,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager
    }
}

impl InputManager {
    pub fn read_line(&self) -> Prompt {
        let str = Self::read_string();
        let sanitized = Self::sanitize_input(&str);
        Self::parse_str(sanitized)
    }

    pub fn clean_screen(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn read_string() -> String {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("can not read user input");
        input
    }

    fn sanitize_input(str: &str) -> &str {
        str.trim()
    }

    fn parse_to_int(str: &str) -> Prompt {
        match str.parse::<i32>() {
            Ok(n) => Prompt::Number(n),
            Err(_err) => Prompt::Text(String::from(str)),
        }
    }

    fn parse_str(str: &str) -> Prompt {
        match str {
            "q" => Prompt::Quit,
            str => Self::parse_to_int(str),
        }
    }
}

impl InputManager {
    pub fn load_data(&self, path: &str) -> Vec<String> {
        let error_msg = format!(
            "Should have been able to read the file at the given path: {}",
            path
        );
        let data = std::fs::read_to_string(path).expect(&error_msg);
        data.lines()
            .map(|l| String::from(Self::sanitize_input(l)))
            .collect()
    }

    pub fn parse_args(&self) -> String {
        let args: Vec<String> = std::env::args().collect();
        let path = args.get(1).expect("Missing path argument!");
        String::from(path)
    }
}
