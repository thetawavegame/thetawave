use bevy::{
    asset::Handle,
    ecs::{
        component::Component,
        entity::Entity,
        query::{Changed, With},
        system::{Commands, Query},
    },
    hierarchy::{BuildChildren, ChildBuilder, Children, DespawnRecursiveExt},
    render::{color::Color, texture::Image},
    ui::{
        node_bundles::{ImageBundle, NodeBundle},
        FlexDirection, Style, UiRect, Val,
    },
    utils::default,
};
use thetawave_interface::{
    abilities::{AbilityCooldownComponent, AbilitySlotIDComponent},
    character::Character,
    health::HealthComponent,
    player::{PlayerComponent, PlayerIDComponent, PlayersResource},
};

use crate::{assets::UiAssets, player::CharactersResource};

use super::parent::PlayerUiChildBuilderExt;

const INNER_PADDING: UiRect = UiRect::all(Val::Percent(5.0));
const INNER_WIDTH: Val = Val::Percent(35.0);
const OUTER_PADDING: UiRect = UiRect::all(Val::Percent(5.0));
const OUTER_WIDTH: Val = Val::Percent(65.0);
const HEALTH_HEIGHT: Val = Val::Percent(55.0);
const HEALTH_COLOR: Color = Color::CRIMSON;
const HEALTH_EMPTY_ALPHA: f32 = 0.05;
const HEALTH_FILLED_ALPHA: f32 = 0.75;
const SHIELDS_HEIGHT: Val = Val::Percent(25.0);
const SHIELDS_COLOR: Color = Color::CYAN;
const SHIELDS_EMPTY_ALPHA: f32 = 0.05;
const SHIELDS_FILLED_ALPHA: f32 = 0.75;
const ARMOR_HEIGHT: Val = Val::Percent(20.0);
const ARMOR_PADDING: UiRect = UiRect {
    left: Val::Percent(0.0),
    right: Val::Percent(0.0),
    top: Val::Vh(0.1),
    bottom: Val::Vh(0.1),
};
const ARMOR_COUNTER_ASPECT_RATIO: Option<f32> = Some(10.0);
const ARMOR_COUNTER_MARGIN: UiRect =
    UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Vh(0.1), Val::Vh(0.1));
const ARMOR_COUNTER_COLOR: Color = Color::GOLD;
const ARMOR_COUNTER_ALPHA: f32 = 0.75;
const ABILITY_VALUE_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.85);

// Player data Uis
#[derive(Component)]
pub(super) struct HealthUi;

#[derive(Component)]
pub(super) struct HealthValueUi;

#[derive(Component)]
pub(super) struct ShieldsUi;

#[derive(Component)]
pub(super) struct ShieldsValueUi;

#[derive(Component)]
pub(super) struct ArmorUi;

#[derive(Component)]
pub(super) struct ArmorCounterUi;

#[derive(Component)]
pub(super) struct AbilitySlotUi;

#[derive(Component)]
pub(super) struct AbilityIconUi;

#[derive(Component)]
pub(super) struct AbilityValueUi;

#[derive(Component)]
pub(super) struct PlayerUi;

#[derive(Component)]
pub(super) struct PlayerInnerUi;

#[derive(Component)]
pub(super) struct PlayerOuterUi;

trait PlayerIDComponentExt {
    fn has_flipped_ui(&self) -> bool;
}

impl PlayerIDComponentExt for PlayerIDComponent {
    /// Determines whether ui should be flipped based on the player ID
    fn has_flipped_ui(&self) -> bool {
        match self {
            PlayerIDComponent::One => false,
            PlayerIDComponent::Two => true,
        }
    }
}

impl PlayerUiChildBuilderExt for ChildBuilder<'_> {
    fn spawn_player_ui(
        &mut self,
        characters_res: &CharactersResource,
        id: PlayerIDComponent,
        players_res: &PlayersResource,
        ui_assets: &UiAssets,
    ) {
        // Only spawn ui for player with id if its player slot is filled
        if let Some(player_data) = &players_res.player_data[id as usize] {
            // Get character for the player slot
            let character = characters_res
                .characters
                .get(&player_data.character)
                .unwrap();

            // Parent player ui node
            self.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            })
            .insert(PlayerUi)
            .with_children(|player| {
                if id.has_flipped_ui() {
                    player.spawn_inner_player_ui(id);
                    player.spawn_outer_player_ui(character, id, ui_assets);
                } else {
                    player.spawn_outer_player_ui(character, id, ui_assets);
                    player.spawn_inner_player_ui(id);
                }
            });
        }
    }

    fn spawn_inner_player_ui(&mut self, id: PlayerIDComponent) {
        self.spawn(NodeBundle {
            style: Style {
                width: INNER_WIDTH,
                height: Val::Percent(100.0),
                padding: INNER_PADDING,
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            ..default()
        })
        .insert(PlayerInnerUi)
        .with_children(|inner| {
            inner
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: HEALTH_HEIGHT,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..default()
                    },
                    background_color: HEALTH_COLOR.with_a(HEALTH_EMPTY_ALPHA).into(),
                    ..default()
                })
                .insert(HealthUi)
                .with_children(|health| {
                    health
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: HEALTH_COLOR.with_a(HEALTH_FILLED_ALPHA).into(),
                            ..default()
                        })
                        .insert(HealthValueUi)
                        .insert(id);
                });

            inner
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: SHIELDS_HEIGHT,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..default()
                    },
                    background_color: SHIELDS_COLOR.with_a(SHIELDS_EMPTY_ALPHA).into(),
                    ..default()
                })
                .insert(ShieldsUi)
                .with_children(|shields_ui| {
                    shields_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: SHIELDS_COLOR.with_a(SHIELDS_FILLED_ALPHA).into(),
                            ..default()
                        })
                        .insert(ShieldsValueUi)
                        .insert(id);
                });

            inner
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: ARMOR_HEIGHT,
                        padding: ARMOR_PADDING,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..default()
                    },
                    ..default()
                })
                .insert(ArmorUi)
                .insert(id);
        });
    }

    fn spawn_outer_player_ui(
        &mut self,
        character: &Character,
        id: PlayerIDComponent,
        ui_assets: &UiAssets,
    ) {
        self.spawn(NodeBundle {
            style: Style {
                width: OUTER_WIDTH,
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::ColumnReverse,
                padding: OUTER_PADDING,
                ..default()
            },
            ..default()
        })
        .insert(PlayerOuterUi)
        .with_children(|outer| {
            // First and bottom ability slot
            outer.spawn_player_ability_slot_ui(
                character,
                id,
                AbilitySlotIDComponent::One,
                id.has_flipped_ui(),
                ui_assets,
            );

            // Second and top ability slot
            outer.spawn_player_ability_slot_ui(
                character,
                id,
                AbilitySlotIDComponent::Two,
                id.has_flipped_ui(),
                ui_assets,
            );
        });
    }

    fn spawn_player_ability_slot_ui(
        &mut self,
        character: &Character,
        player_id: PlayerIDComponent,
        ability_slot_id: AbilitySlotIDComponent,
        is_flipped: bool,
        ui_assets: &UiAssets,
    ) {
        let ability_slot_image = ui_assets.get_ability_slot_image(is_flipped).clone();

        self.spawn(ImageBundle {
            image: ability_slot_image.into(),
            style: Style {
                width: Val::Percent(100.0),
                aspect_ratio: Some(1.0),
                ..default()
            },
            ..default()
        })
        .insert(AbilitySlotUi)
        .insert(ability_slot_id)
        .insert(player_id)
        .with_children(|ability_slot| match &ability_slot_id {
            AbilitySlotIDComponent::One => {
                if let Some(slot_1_ability) = &character.slot_1_ability {
                    ability_slot.spawn_player_ability_icon_ui(
                        player_id,
                        ability_slot_id,
                        ui_assets.get_slot_1_ability_image(slot_1_ability),
                    );
                }
            }
            AbilitySlotIDComponent::Two => {
                if let Some(slot_2_ability) = &character.slot_2_ability {
                    ability_slot.spawn_player_ability_icon_ui(
                        player_id,
                        ability_slot_id,
                        ui_assets.get_slot_2_ability_image(slot_2_ability),
                    );
                }
            }
        });
    }

    fn spawn_player_ability_icon_ui(
        &mut self,
        player_id: PlayerIDComponent,
        ability_slot_id: AbilitySlotIDComponent,
        image: Handle<Image>,
    ) {
        self.spawn(ImageBundle {
            image: image.into(),
            style: Style {
                width: Val::Percent(100.0),
                aspect_ratio: Some(1.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(AbilityIconUi)
        .insert(player_id)
        .insert(ability_slot_id)
        .with_children(|ability_icon| {
            ability_icon
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: ABILITY_VALUE_COLOR.into(),
                    ..default()
                })
                .insert(player_id)
                .insert(ability_slot_id)
                .insert(AbilityValueUi);
        });
    }

    fn spawn_player_armor_counter_ui(&mut self) {
        self.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                aspect_ratio: ARMOR_COUNTER_ASPECT_RATIO,
                margin: ARMOR_COUNTER_MARGIN,
                ..default()
            },
            background_color: ARMOR_COUNTER_COLOR.with_a(ARMOR_COUNTER_ALPHA).into(),
            ..default()
        })
        .insert(ArmorCounterUi);
    }
}

pub(super) fn update_player_abilities_ui_system(
    player_query: Query<(&Children, &PlayerIDComponent), With<PlayerComponent>>,
    player_ability_query: Query<(&AbilityCooldownComponent, &AbilitySlotIDComponent)>,
    mut ability_ui_query: Query<
        (&mut Style, &AbilitySlotIDComponent, &PlayerIDComponent),
        With<AbilityValueUi>,
    >,
) {
    for (mut style, ui_ability_slot_id, ability_slot_player_id) in ability_ui_query.iter_mut() {
        for (player_children, player_id) in player_query.iter() {
            if *player_id == *ability_slot_player_id {
                for child in player_children.iter() {
                    if let Ok((ability_cooldown, ability_slot_id)) =
                        player_ability_query.get(*child)
                    {
                        if *ability_slot_id == *ui_ability_slot_id {
                            style.height = Val::Percent(
                                100.0 * ability_cooldown.cooldown_timer.fraction_remaining(),
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Updates each player's health bar ui
pub(super) fn update_player_health_ui_system(
    player_query: Query<(&HealthComponent, &PlayerIDComponent), Changed<HealthComponent>>,
    mut health_ui: Query<(&mut Style, &PlayerIDComponent), With<HealthValueUi>>,
) {
    for (player_health, player_id) in player_query.iter() {
        for (mut style, health_id) in health_ui.iter_mut() {
            if player_id == health_id {
                style.height = Val::Percent(100.0 * player_health.get_health_percentage());
            }
        }
    }
}

/// Updates each player's shields bar ui
pub(super) fn update_player_shields_ui_system(
    player_query: Query<(&HealthComponent, &PlayerIDComponent), Changed<HealthComponent>>,
    mut shields_ui: Query<(&mut Style, &PlayerIDComponent), With<ShieldsValueUi>>,
) {
    for (player_health, player_id) in player_query.iter() {
        for (mut style, shields_id) in shields_ui.iter_mut() {
            if player_id == shields_id {
                style.height = Val::Percent(100.0 * player_health.get_shields_percentage());
            }
        }
    }
}

/// Updates each player's armor ui
pub(super) fn update_player_armor_ui_system(
    mut commands: Commands,
    player_query: Query<(&HealthComponent, &PlayerIDComponent), Changed<HealthComponent>>,
    armor_ui: Query<(Entity, &PlayerIDComponent), With<ArmorUi>>,
) {
    for (player_health, player_id) in player_query.iter() {
        for (entity, armor_id) in armor_ui.iter() {
            if player_id == armor_id {
                // spawn all of the existing child armor ticks
                commands.entity(entity).despawn_descendants();

                // spawn armor ticks
                commands.entity(entity).with_children(|armor_ui| {
                    for _ in 0..player_health.get_armor() {
                        armor_ui.spawn_player_armor_counter_ui();
                    }
                });
            }
        }
    }
}
