mod input_manager;
mod participant_manager;

use input_manager::{InputManager, Prompt};
use participant_manager::ParticipantManager;

fn main() {
    let p = vec![
        String::from("David"),
        String::from("Nacho"),
        String::from("Mafe"),
        String::from("Samuel"),
    ];

    let input_manager: InputManager = InputManager::new();
    let mut participant_manager = ParticipantManager::new(p);

    // print time and date
    println!("Type an id to mark an user that has already spoken");
    loop {
        participant_manager.display_current();
        let prompt = input_manager.read_line();
        match prompt {
            Prompt::Number(id) => {
                participant_manager.find_participant(id);
            }
            _ => break,
        };
        input_manager.clean_screen();
    }
    // end at
}
