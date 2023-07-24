struct Participant {
    name: String,
    has_spoken: bool,
    id: i32,
}

impl Participant {
    fn new(name: String, id: i32) -> Self {
        Participant {
            name,
            has_spoken: false,
            id,
        }
    }

    fn format(&self) {
        // display info
        let f = format!("");
    }

    fn flip_spoken(id: i32) {

    }
}

struct ParticipantManager {
    participants: Vec<Participant>,
}

impl ParticipantManager {
    fn new(vec: Vec<String>) -> Self {
        let p = vec
            .iter()
            .enumerate()
            .map(|(i, n)| Participant::new(n.clone(), i as i32))
            .collect();
        ParticipantManager { participants: p }
    }

    // display participants
    fn display_current(&self) {
        self.participants.iter().for_each(|p| p.format())
    }

    // mark participant that spoke

    //
}
fn main() {
    println!("Hello, world!");
}
