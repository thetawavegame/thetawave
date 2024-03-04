use bevy::ecs::system::{Query, Res};
use bevy::time::Time;
use thetawave_interface::player::PlayerAbilityCooldownsComponent;

/// Tick ability cooldown timers for each player
pub fn player_ability_cooldown_system(
    mut player_query: Query<&mut PlayerAbilityCooldownsComponent>,
    time: Res<Time>,
) {
    for mut ability_cooldowns in player_query.iter_mut() {
        // Tick each timer if it is Some
        for cooldown in ability_cooldowns
            .cooldowns
            .iter_mut()
            .filter_map(|maybe_cooldown| maybe_cooldown.as_mut())
        {
            cooldown.tick(time.delta());
        }
    }
}
