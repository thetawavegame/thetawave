use bevy::ecs::{query::Changed, system::Query};
use thetawave_interface::player::{PlayerInventoryComponent, PlayerOutgoingDamageComponent};

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
