use bevy::ecs::{query::Changed, system::Query};
use thetawave_interface::player::{PlayerInventoryComponent, PlayerOutgoingDamageComponent};

trait PlayerOutgoingDamageComponentExt {
    fn update_cooldown_multiplier_from_collected_money(&mut self, money: usize);
}

impl PlayerOutgoingDamageComponentExt for PlayerOutgoingDamageComponent {
    /// Updates the `cooldown_multilier` using the `base_cooldown_multiplier` and a money parameter
    /// along an exponential decay curve
    fn update_cooldown_multiplier_from_collected_money(&mut self, money: usize) {
        self.cooldown_multiplier =
            1.0 + (self.base_cooldown_multiplier - 1.0) * f32::exp(-0.1 * money as f32);
    }
}

/// Updates the player's cooldown multiplier everytime the money in the `PlayerInventoryComponent` changes
pub(in crate::player) fn scale_ability_cooldowns_system(
    mut player_query: Query<
        (
            &mut PlayerOutgoingDamageComponent,
            &PlayerInventoryComponent,
        ),
        Changed<PlayerInventoryComponent>,
    >,
) {
    for (mut player_damage, player_inventory) in player_query.iter_mut() {
        player_damage.update_cooldown_multiplier_from_collected_money(player_inventory.money);
    }
}
