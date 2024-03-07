use bevy::ecs::event::EventWriter;
use bevy::ecs::query::With;
use bevy::ecs::system::{Query, Res};
use bevy::hierarchy::Children;
use bevy::log::info;
use bevy::time::Time;
use leafwing_input_manager::action_state::ActionState;
use thetawave_interface::abilities::{
    AbilityCooldownComponent, AbilitySlotIDComponent, ActivateAbilityEvent,
};
use thetawave_interface::input::PlayerAction;
use thetawave_interface::player::{PlayerComponent, PlayerIDComponent};

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
