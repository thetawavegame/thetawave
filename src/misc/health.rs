use bevy::prelude::warn;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Health {
    max_health: f32,
    health: f32,
    #[serde(default)]
    armor: usize,
    #[serde(default)]
    shields: f32,
    #[serde(default)]
    max_shields: f32,
    #[serde(default)]
    shields_recharge_rate: f32,
}

impl Health {
    #[allow(dead_code)]
    /// Create a new health struct from a maximum health value
    pub fn new(health: f32, shields: f32, shields_recharge_rate: f32) -> Self {
        Health {
            max_health: health,
            health,
            max_shields: shields,
            shields,
            armor: 0,
            shields_recharge_rate,
        }
    }

    pub fn regenerate_shields(&mut self) {
        self.shields += self.shields_recharge_rate;
        if self.shields > self.max_shields {
            self.shields = self.max_shields;
        }
    }

    /// Check if health is below zero
    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    /// Take damage (deplete armore, then shields, then health  in that order)
    pub fn take_damage(&mut self, damage: f32) {
        if self.armor == 0 {
            if self.shields > 0.0 {
                self.shields -= damage;
                if self.shields < 0.0 {
                    let remaining_damage = -self.shields;
                    self.health -= remaining_damage;
                    self.shields = 0.0;
                }
            } else {
                self.health -= damage;
                if self.health < 0.0 {
                    self.health = 0.0;
                }
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

    /// Get maximum health
    pub fn get_max_shields(&self) -> f32 {
        self.max_shields
    }

    /// Get current health
    pub fn get_shields(&self) -> f32 {
        self.shields
    }

    /// Get available armor count
    pub fn get_armor(&self) -> usize {
        self.armor
    }

    /// Set health to value
    #[allow(dead_code)]
    pub fn set_health(&mut self, health: f32) {
        if health > self.max_health {
            warn!(
                "Attempting to set health value to value above maximum health!
            Setting to max value instead."
            );
            self.health = self.max_health;
        } else {
            self.health = health;
        }
    }

    #[allow(dead_code)]
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
