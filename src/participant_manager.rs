struct Participant {
    name: String,
    has_spoken: bool,
    id: i32,
}

impl Participant {
    pub fn new(name: String, id: i32) -> Self {
        Participant {
            name,
            id,
            has_spoken: false,
        }
    }

    fn format(&self) {
        let symbol = if self.has_spoken { "[X]" } else { "[ ]" };
        let f = format!("{}. {} {}", self.id, self.name, symbol);
        println!("{}", f);
    }

    fn flip_spoken(&mut self) {
        self.has_spoken = !self.has_spoken;
    }
}

pub struct ParticipantManager {
    participants: Vec<Participant>,
}

impl ParticipantManager {
    pub fn new(vec: Vec<String>) -> Self {
        let p = vec
            .iter()
            .enumerate()
            .map(|(i, n)| Participant::new(n.clone(), (i + 1) as i32))
            .collect();
        ParticipantManager { participants: p }
    }

    pub fn display_current(&self) {
        self.participants.iter().for_each(|p| p.format());
    }

    pub fn find_participant(&mut self, id: i32) {
        let found = self.participants.iter_mut().find(|p| p.id.eq(&id));
        if let Some(p) = found {
            p.flip_spoken();
        }
    }
}
