mod input_manager;
mod participant_manager;

use input_manager::{InputManager, Prompt};
use participant_manager::ParticipantManager;

fn main() {
    let input_manager: InputManager = InputManager::new();
    let data = input_manager.load_data("./data.txt");
    let mut participant_manager = ParticipantManager::new(data);

    loop {
        // print time and date
        println!("Type an id to mark an user that has already spoken");
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
