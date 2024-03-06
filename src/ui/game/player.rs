use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        entity::Entity,
        query::{Changed, Or, With},
        system::{Commands, ParamSet, Query},
    },
    hierarchy::{BuildChildren, ChildBuilder, DespawnRecursiveExt},
    render::color::Color,
    ui::{
        node_bundles::{ImageBundle, NodeBundle},
        BackgroundColor, FlexDirection, Style, UiRect, Val,
    },
    utils::default,
};
use thetawave_interface::{
    abilities::AbilitySlotIDComponent,
    character::CharacterType,
    health::HealthComponent,
    player::{PlayerIDComponent, PlayersResource},
};

use super::parent::PlayerUiChildBuilderExt;

// Player data Uis
#[derive(Component)]
pub struct HealthUi;

#[derive(Component)]
pub struct HealthValueUi;

#[derive(Component)]
pub struct ShieldsUi;

#[derive(Component)]
pub struct ShieldsValueUi;

#[derive(Component)]
pub struct ArmorUi;

#[derive(Component)]
pub struct ArmorCounterUi;

#[derive(Component)]
pub struct AbilitySlotUi;

#[derive(Component)]
pub struct AbilityIconUi;

#[derive(Component)]
pub struct PlayerUi;

#[derive(Component)]
pub struct PlayerInnerUi;

#[derive(Component)]
pub struct PlayerOuterUi;

impl PlayerUiChildBuilderExt for ChildBuilder<'_> {
    fn spawn_player_ui(
        &mut self,
        id: PlayerIDComponent,
        players_res: &PlayersResource,
        asset_server: &AssetServer,
    ) {
        // Only spawn ui for player with id if its player slot is filled
        if let Some(player_data) = &players_res.player_data[id as usize] {
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
                    player.spawn_outer_player_ui(id, asset_server);
                } else {
                    player.spawn_outer_player_ui(id, asset_server);
                    player.spawn_inner_player_ui(id);
                }
            });
        }
    }

    fn spawn_inner_player_ui(&mut self, id: PlayerIDComponent) {
        self.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(35.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Percent(5.0)),
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
                        height: Val::Percent(55.0),
                        flex_direction: FlexDirection::ColumnReverse,
                        ..default()
                    },
                    background_color: Color::CRIMSON.with_a(0.05).into(),
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
                            background_color: Color::CRIMSON.with_a(0.75).into(),
                            ..default()
                        })
                        .insert(HealthValueUi)
                        .insert(id);
                });

            inner
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(25.0),
                        flex_direction: FlexDirection::ColumnReverse,
                        ..default()
                    },
                    background_color: Color::CYAN.with_a(0.05).into(),
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
                            background_color: Color::CYAN.with_a(0.75).into(),
                            ..default()
                        })
                        .insert(ShieldsValueUi)
                        .insert(id);
                });

            inner
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(20.0),
                        padding: UiRect::new(
                            Val::Percent(0.0),
                            Val::Percent(0.0),
                            Val::Vh(0.1),
                            Val::Vh(0.1),
                        ),
                        flex_direction: FlexDirection::ColumnReverse,
                        ..default()
                    },
                    ..default()
                })
                .insert(ArmorUi)
                .insert(id);
        });
    }

    fn spawn_outer_player_ui(&mut self, id: PlayerIDComponent, asset_server: &AssetServer) {
        self.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(65.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::ColumnReverse,
                padding: UiRect::all(Val::Percent(5.0)),
                ..default()
            },
            ..default()
        })
        .insert(PlayerOuterUi)
        .with_children(|outer| {
            // First and bottom ability slot
            outer.spawn_player_ability_slot_ui(
                id,
                AbilitySlotIDComponent::One,
                id.has_flipped_ui(),
                &asset_server,
            );

            // Second and top ability slot
            outer.spawn_player_ability_slot_ui(
                id,
                AbilitySlotIDComponent::Two,
                id.has_flipped_ui(),
                &asset_server,
            );
        });
    }

    fn spawn_player_ability_slot_ui(
        &mut self,
        player_id: PlayerIDComponent,
        ability_slot_id: AbilitySlotIDComponent,
        is_flipped: bool,
        asset_server: &AssetServer,
    ) {
        let ability_slot_image = asset_server.load(if is_flipped {
            "texture/ability_square_right.png"
        } else {
            "texture/ability_square_left.png"
        });

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
        .with_children(|ability_slot| {});
    }
}

fn build_armor_counter(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                aspect_ratio: Some(10.0),
                margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Vh(0.1), Val::Vh(0.1)),
                ..default()
            },
            background_color: Color::GOLD.with_a(0.75).into(),
            ..default()
        })
        .insert(ArmorCounterUi);
}

pub fn update_player_ui_system(
    mut commands: Commands,
    player_query: Query<(&HealthComponent, &PlayerIDComponent), Changed<HealthComponent>>,
    mut player_ui: ParamSet<(
        Query<(&mut Style, &PlayerIDComponent), With<HealthValueUi>>,
        Query<(&mut Style, &PlayerIDComponent), With<ShieldsValueUi>>,
        Query<(Entity, &PlayerIDComponent), With<ArmorUi>>,
        Query<(&mut Style, &AbilitySlotIDComponent, &PlayerIDComponent)>,
    )>,
) {
    for (player_health, player_id) in player_query.iter() {
        // health ui
        for (mut style, health_id) in player_ui.p0().iter_mut() {
            if player_id == health_id {
                style.height = Val::Percent(100.0 * player_health.get_health_percentage());
            }
        }

        // shields ui
        for (mut style, shields_id) in player_ui.p1().iter_mut() {
            if player_id == shields_id {
                style.height = Val::Percent(100.0 * player_health.get_shields_percentage());
            }
        }

        // armor ui
        for (entity, armor_id) in player_ui.p2().iter() {
            if player_id == armor_id {
                // spawn all of the existing child armor ticks
                commands.entity(entity).despawn_descendants();

                // spawn armor ticks
                commands.entity(entity).with_children(|armor_ui| {
                    for _ in 0..player_health.get_armor() {
                        build_armor_counter(armor_ui);
                    }
                });
            }
        }
    }
}
