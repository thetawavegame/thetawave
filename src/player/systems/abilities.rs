use bevy::ecs::system::{Query, Res};
use bevy::time::Time;
use thetawave_interface::abilities::AbilityCooldownComponent;

/// Tick ability cooldown timers for each player
pub fn player_ability_cooldown_system(
    mut player_query: Query<&mut AbilityCooldownComponent>,
    time: Res<Time>,
) {
    for mut ability_cooldowns in player_query.iter_mut() {
        ability_cooldowns.0.tick(time.delta());
    }
}
