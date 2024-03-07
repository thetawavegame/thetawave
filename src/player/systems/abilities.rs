use bevy::ecs::entity::Entity;
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::query::With;
use bevy::ecs::system::{Query, Res};
use bevy::hierarchy::Children;
use bevy::log::info;
use bevy::prelude::default;
use bevy::time::Time;
use bevy::transform::components::Transform;
use bevy_rapier2d::dynamics::Velocity;
use leafwing_input_manager::action_state::ActionState;
use thetawave_interface::abilities::{
    AbilityCooldownComponent, AbilitySlotIDComponent, ActivateAbilityEvent,
    StandardWeaponAbilityComponent,
};
use thetawave_interface::input::PlayerAction;
use thetawave_interface::player::{
    self, PlayerComponent, PlayerIDComponent, PlayerOutgoingDamageComponent,
};
use thetawave_interface::weapon::WeaponProjectileData;

use crate::spawnable::{FireWeaponEvent, InitialMotion};

/// Tick ability cooldown timers for each player
pub fn player_ability_cooldown_system(
    mut ability_query: Query<&mut AbilityCooldownComponent>,
    time: Res<Time>,
) {
    for mut ability_cooldowns in ability_query.iter_mut() {
        ability_cooldowns.0.tick(time.delta());
    }
}

/// Checks all abilities for if their cooldown timers are finished, if they are,
/// and the player has the ability's respective input pressed, sends an ActivateAbilityEvent
/// and resets the ability's cooldown timer
pub fn player_ability_input_system(
    player_input_query: Query<(&ActionState<PlayerAction>, &PlayerIDComponent, &Children)>,
    mut ability_query: Query<(&mut AbilityCooldownComponent, &AbilitySlotIDComponent)>,
    mut ability_event_writer: EventWriter<ActivateAbilityEvent>,
) {
    for (action_state, player_id, children) in player_input_query.iter() {
        for child in children {
            if let Ok((mut ability_cooldown, ability_id)) = ability_query.get_mut(*child) {
                match ability_id {
                    AbilitySlotIDComponent::One => {
                        if action_state.pressed(&PlayerAction::SlotOneAbility)
                            && ability_cooldown.0.finished()
                        {
                            ability_cooldown.0.reset();
                            ability_event_writer
                                .send(ActivateAbilityEvent::new(*player_id, *ability_id));
                        }
                    }
                    AbilitySlotIDComponent::Two => {
                        if action_state.pressed(&PlayerAction::SlotTwoAbility)
                            && ability_cooldown.0.finished()
                        {
                            ability_cooldown.0.reset();
                            ability_event_writer
                                .send(ActivateAbilityEvent::new(*player_id, *ability_id));
                        }
                    }
                }
            }
        }
    }
}

/// Activates a standard waeapon ability for a player for corresponding ActivateAbilityEvents
/// Combines that outgoing damage stats of the player with the stats of the weapon
pub fn standard_weapon_ability_system(
    player_query: Query<(
        Entity,
        &Transform,
        &Velocity,
        &PlayerOutgoingDamageComponent,
        &PlayerIDComponent,
        &Children,
    )>,
    ability_query: Query<(&AbilitySlotIDComponent, &StandardWeaponAbilityComponent)>,
    mut ability_event_reader: EventReader<ActivateAbilityEvent>,
    mut fire_weapon_event_writer: EventWriter<FireWeaponEvent>,
) {
    for event in ability_event_reader.read() {
        for (
            player_entity,
            player_transform,
            player_velocity,
            player_damage,
            player_id,
            children,
        ) in player_query.iter()
        {
            for child in children.iter() {
                if let Ok((ability_id, weapon)) = ability_query.get(*child) {
                    if event.player_id == *player_id && event.ability_slot_id == *ability_id {
                        fire_weapon_event_writer.send(FireWeaponEvent {
                            weapon_projectile_data: WeaponProjectileData {
                                ammunition: weapon.ammunition,
                                damage: (weapon.damage_multiplier
                                    * player_damage.weapon_damage as f32)
                                    as usize,
                                position: player_damage.projectile_spawn_position,
                                speed: weapon.speed_multiplier * player_damage.projectile_speed,
                                direction: weapon.direction,
                                despawn_time: weapon.despawn_time_multiplier
                                    * player_damage.projectile_despawn_time,
                                count: ((weapon.count_multiplier
                                    * player_damage.projectile_count as f32)
                                    as usize)
                                    .max(1),
                                spread_pattern: weapon.spread_pattern.clone(),
                                size: weapon.size_multiplier * player_damage.projectile_size,
                                sound: weapon.sound,
                            },
                            source_transform: *player_transform,
                            source_entity: player_entity,
                            initial_motion: InitialMotion {
                                linvel: Some(player_velocity.linvel),
                                ..default()
                            },
                        });
                    }
                }
            }
        }
    }
}
