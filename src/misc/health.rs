use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Health {
    max_health: f32,
    health: f32,
    armor: usize,
}

impl Health {
    /// Create a new health struct from a maximum health value
    pub fn new(health: f32) -> Self {
        Health {
            max_health: health,
            health,
            armor: 0,
        }
    }

    /// Check if health is below zero
    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    /// Take damage (remove armor first if available)
    pub fn take_damage(&mut self, damage: f32) {
        if self.armor == 0 {
            self.health -= damage;
            if self.health < 0.0 {
                self.health = 0.0;
            }
        } else {
            self.armor -= 1;
        }
    }

    /// Get maximum health
    pub fn get_max_health(&self) -> f32 {
        self.max_health
    }

    /// Get current health
    pub fn get_health(&self) -> f32 {
        self.health
    }

    /// Get available armor count
    pub fn get_armor(&self) -> usize {
        self.armor
    }

    /// Set health to value
    pub fn set_health(&mut self, health: f32) {
        if health > self.max_health {
            eprintln!(
                "Attempting to set health value to value above maximum health!
            Setting to max value instead."
            );
            self.health = self.max_health;
        } else {
            self.health = health;
        }
    }

    /// Set maximum health to value
    pub fn set_max_health(&mut self, max_health: f32) {
        if max_health <= 0.0 {
            panic!("Attempted to set maximum health to value less than or equal to 0.0!");
        }

        self.max_health = max_health;

        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }

    /// Add to health
    pub fn heal(&mut self, health: f32) {
        if health < 0.0 {
            panic!("Attempted to heal by negative value. Use take_damage function instead?");
        }

        self.health += health;
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }

    /// Add to armor
    pub fn gain_armor(&mut self, armor: usize) {
        self.armor += armor;
    }
}
