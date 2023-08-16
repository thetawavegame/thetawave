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
    defense: f32,
    /// Maximum defense
    max_defense: f32,
}

impl DefenseData {
    /// Returns if the defense objective is failed
    pub fn is_failed(&self) -> bool {
        self.defense <= 0.0
    }

    /// "Heal" defense
    pub fn gain_defense(&mut self, value: f32) {
        if value < 0.0 {
            eprintln!(
                "Attempted to add a negative value to defense. Use take_damage function instead."
            );
        } else {
            self.defense = (self.defense + value).min(self.max_defense);
        }
    }

    /// Decrement defense level
    pub fn take_damage(&mut self, value: f32) {
        if value < 0.0 {
            eprintln!(
                "Attempted to take a negative amount of damage. Use gain_defense function instead."
            );
        } else {
            self.defense = (self.defense - value).max(0.0);
        }
    }

    /// Percentage of defense left
    pub fn get_percentage_left(&self) -> f32 {
        self.defense / self.max_defense
    }
}
