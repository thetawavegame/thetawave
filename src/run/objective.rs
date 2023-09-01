use serde::Deserialize;

/// Additional fail condition for a level
#[derive(Deserialize, Clone, Debug)]
pub enum Objective {
    /// Objective representing defense of a planet, structure, etc
    Defense(DefenseData),
}

/// Tracks data for the defense objective
#[derive(Deserialize, Clone, Debug)]
pub struct DefenseData {
    /// Current defense
    pub defense: usize,
    /// Maximum defense
    pub max_defense: usize,
}

impl DefenseData {
    /// Returns if the defense objective is failed
    pub fn is_failed(&self) -> bool {
        self.defense == 0
    }

    /// "Heal" defense
    pub fn gain_defense(&mut self, value: usize) {
        self.defense = (self.defense + value).min(self.max_defense);
    }

    /// Decrement defense level
    pub fn take_damage(&mut self, value: usize) {
        self.defense = self.defense.saturating_sub(value);
    }

    /// Percentage of defense left
    pub fn get_percentage(&self) -> f32 {
        if self.max_defense > 0 {
            self.defense as f32 / self.max_defense as f32
        } else {
            0.0
        }
    }
}
