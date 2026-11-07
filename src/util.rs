//! jitsi model — video conferencing.

#[derive(Debug, Clone)]
pub struct Room {
    pub name: String,
    pub subject: String,
    pub participants: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub rooms: Vec<Room>,
}

impl Default for Model {
    fn default() -> Self { Self { rooms: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create_room(&mut self, name: &str, subject: &str) -> bool {
        if !self.rooms.iter().any(|r| r.name == name) {
            self.rooms.push(Room { name: name.into(), subject: subject.into(), participants: vec![] });
            true
        } else { false }
    }

    pub fn join(&mut self, room: &str, user: &str) -> bool {
        if let Some(r) = self.rooms.iter_mut().find(|r| r.name == room) {
            r.participants.push(user.into());
            true
        } else { false }
    }

    pub fn leave(&mut self, room: &str, user: &str) -> bool {
        if let Some(r) = self.rooms.iter_mut().find(|r| r.name == room) {
            if let Some(pos) = r.participants.iter().position(|p| p == user) { r.participants.remove(pos); true } else { false }
        } else { false }
    }

    pub fn list_rooms(&self) -> &[Room] { &self.rooms }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn room_join_leave() {
        let mut j = Model::new();
        assert!(j.create_room("standup", "Daily"));
        assert!(j.join("standup", "alice"));
        assert!(j.join("standup", "bob"));
        assert!(j.leave("standup", "alice"));
    }
}
