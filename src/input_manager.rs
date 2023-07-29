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
    pub fn load_data(&self, file_path: &str) -> Vec<String> {
        vec![
            String::from("David"),
            String::from("Nacho"),
            String::from("Mafe"),
            String::from("Samuel"),
        ]
    }
}