use bevy::ecs::entity::Entity;
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::system::{Query, Res};
use bevy::hierarchy::Children;
use bevy::math::Vec2;
use bevy::prelude::default;
use bevy::time::{Time, Timer, TimerMode};
use bevy::transform::components::Transform;
use bevy_rapier2d::dynamics::{ExternalImpulse, Velocity};
use leafwing_input_manager::action_state::ActionState;
use thetawave_interface::abilities::{
    AbilityCooldownComponent, AbilitySlotIDComponent, ActivateAbilityEvent, ChargeAbilityComponent,
    StandardWeaponAbilityComponent,
};
use thetawave_interface::input::PlayerAction;
use thetawave_interface::player::{
    PlayerIDComponent, PlayerIncomingDamageComponent, PlayerMovementComponent,
    PlayerOutgoingDamageComponent,
};
use thetawave_interface::weapon::WeaponProjectileData;

use crate::spawnable::{FireWeaponEvent, InitialMotion};

/// Tick ability cooldown timers for each player
pub fn player_ability_cooldown_system(
    mut ability_query: Query<&mut AbilityCooldownComponent>,
    time: Res<Time>,
) {
    for mut ability_cooldowns in ability_query.iter_mut() {
        ability_cooldowns.cooldown_timer.tick(time.delta());
    }
}

/// Checks all abilities for if their cooldown timers (in `AbilityCooldownComponent`) are finished, if they are,
/// and the player has the ability's respective input pressed, sends an ActivateAbilityEvent
/// and resets the ability's cooldown timer
pub fn player_ability_input_system(
    player_input_query: Query<(
        &ActionState<PlayerAction>,
        &PlayerOutgoingDamageComponent,
        &PlayerIDComponent,
        &Children,
    )>,
    mut ability_query: Query<(&mut AbilityCooldownComponent, &AbilitySlotIDComponent)>,
    mut ability_event_writer: EventWriter<ActivateAbilityEvent>,
) {
    for (action_state, player_damage, player_id, children) in player_input_query.iter() {
        for child in children {
            if let Ok((mut ability_cooldown, ability_id)) = ability_query.get_mut(*child) {
                match ability_id {
                    AbilitySlotIDComponent::One => {
                        if action_state.pressed(&PlayerAction::SlotOneAbility)
                            && ability_cooldown.cooldown_timer.finished()
                        {
                            ability_cooldown.cooldown_timer = Timer::from_seconds(
                                ability_cooldown.base_cooldown_time
                                    * player_damage.cooldown_multiplier,
                                TimerMode::Once,
                            );
                            ability_event_writer
                                .send(ActivateAbilityEvent::new(*player_id, *ability_id));
                        }
                    }
                    AbilitySlotIDComponent::Two => {
                        if action_state.pressed(&PlayerAction::SlotTwoAbility)
                            && ability_cooldown.cooldown_timer.finished()
                        {
                            ability_cooldown.cooldown_timer = Timer::from_seconds(
                                ability_cooldown.base_cooldown_time
                                    * player_damage.cooldown_multiplier,
                                TimerMode::Once,
                            );
                            ability_event_writer
                                .send(ActivateAbilityEvent::new(*player_id, *ability_id));
                        }
                    }
                }
            }
        }
    }
}

/// Activates a standard waeapon ability (abilities with `StandardWeaponAbilityComponent`)
/// for a player for corresponding ActivateAbilityEvents.
/// Combines the stats in the player's `PlayerOutgoingDamageComponent` of the player with
/// the stats in `StandardWeaponAbilityComponent`.
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
                                    .round() as usize,
                                position: player_damage.projectile_spawn_position,
                                speed: weapon.speed_multiplier * player_damage.projectile_speed,
                                direction: weapon.direction,
                                despawn_time: weapon.despawn_time_multiplier
                                    * player_damage.projectile_despawn_time,
                                count: ((weapon.count_multiplier
                                    * player_damage.projectile_count as f32)
                                    .round() as usize)
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

/// Activates a charge ability (abilities with `ChargeAbilityComponent`)
/// for a player for corresponding ActivateAbilityEvents.
/// Applies damage reduction and an external impulse to the player
pub fn start_charge_ability_system(
    mut player_query: Query<(
        &ActionState<PlayerAction>,
        &mut ExternalImpulse,
        &mut PlayerMovementComponent,
        &mut PlayerIncomingDamageComponent,
        &PlayerIDComponent,
        &Children,
    )>,
    mut ability_query: Query<(&AbilitySlotIDComponent, &mut ChargeAbilityComponent)>,
    mut ability_event_reader: EventReader<ActivateAbilityEvent>,
) {
    for event in ability_event_reader.read() {
        for (
            action_state,
            mut player_ext_impulse,
            mut player_movement,
            mut player_incoming_damage,
            player_id,
            children,
        ) in player_query.iter_mut()
        {
            for child in children.iter() {
                if let Ok((ability_id, mut charge_ability)) = ability_query.get_mut(*child) {
                    if event.player_id == *player_id && event.ability_slot_id == *ability_id {
                        // check all movement inputs to see if the player wants to charge in a specific direction
                        let up = action_state.pressed(&PlayerAction::MoveUp);
                        let down = action_state.pressed(&PlayerAction::MoveDown);
                        let left = action_state.pressed(&PlayerAction::MoveLeft);
                        let right = action_state.pressed(&PlayerAction::MoveRight);

                        // try to create a normalized Vec2 using the inputs
                        if let Some(vec2_normal) = Vec2::new(
                            (-(left as i8) + right as i8) as f32,
                            (-(down as i8) + up as i8) as f32,
                        )
                        .try_normalize()
                        {
                            // multiply the normalized vector by the charge ability's impulse
                            player_ext_impulse.impulse = charge_ability.impulse * vec2_normal;
                        } else {
                            // if a normalized vector could not be created apply the impulse in the +y direction
                            player_ext_impulse.impulse = Vec2::new(0.0, charge_ability.impulse);
                        }

                        // disable movement and apply damage reduction
                        player_movement.movement_enabled = false;
                        player_incoming_damage.multiplier -=
                            charge_ability.incoming_damage_multiplier;

                        // begin the action timer for the ability
                        charge_ability.action_timer.reset();
                    }
                }
            }
        }
    }
}

/// Updates the charge ability (`ChargeAbilityComponent`)
/// Ticks the action timer, when completed enables movment and resets the incoming damage multiplier
pub fn update_charge_ability_system(
    mut player_query: Query<(
        &mut Velocity,
        &mut PlayerMovementComponent,
        &mut PlayerIncomingDamageComponent,
        &Children,
    )>,
    mut ability_query: Query<&mut ChargeAbilityComponent>,
    time: Res<Time>,
) {
    for (mut player_velocity, mut player_movement, mut player_incoming_damage, children) in
        player_query.iter_mut()
    {
        for child in children.iter() {
            if let Ok(mut charge_ability) = ability_query.get_mut(*child) {
                charge_ability.action_timer.tick(time.delta());

                // when the action timer is completed reverse the damage reduction, enable movement
                // set the velocity of the player to 0
                if charge_ability.action_timer.just_finished() {
                    player_velocity.linvel = Vec2::splat(0.0);
                    player_movement.movement_enabled = true;
                    player_incoming_damage.multiplier += charge_ability.incoming_damage_multiplier;
                }
            }
        }
    }
}
