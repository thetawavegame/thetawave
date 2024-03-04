use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        entity::Entity,
        query::{Changed, With},
        system::{Commands, ParamSet, Query},
    },
    hierarchy::{BuildChildren, ChildBuilder, DespawnRecursiveExt},
    render::color::Color,
    ui::{
        node_bundles::{ImageBundle, NodeBundle},
        FlexDirection, Style, UiRect, Val,
    },
    utils::default,
};
use thetawave_interface::{
    character::CharacterType,
    health::HealthComponent,
    player::{PlayerAbilitiesComponent, PlayerIDComponent, PlayersResource},
    weapon::WeaponComponent,
};

// Player data Uis
#[derive(Component)]
pub struct HealthUI;

#[derive(Component)]
pub struct HealthValueUI;

#[derive(Component)]
pub struct ShieldsUI;

#[derive(Component)]
pub struct ShieldsValueUI;

#[derive(Component)]
pub struct ArmorUI;

#[derive(Component)]
pub struct ArmorCounterUI;

#[derive(Component)]
pub struct AbilitySlotUI {
    pub ability_index: usize,
}

#[derive(Component)]
pub struct AbilityIconUI;

#[derive(Component)]
pub struct AbilityValueUI {
    pub ability_index: usize,
}

#[derive(Component)]
pub struct PlayerUI;

#[derive(Component)]
pub struct PlayerInnerUI;

#[derive(Component)]
pub struct PlayerOuterUI;

pub fn build_player_ui(
    player_id: PlayerIDComponent,
    parent: &mut ChildBuilder,
    players_resource: &PlayersResource,
    asset_server: &AssetServer,
) {
    if let Some(player_data) = &players_resource.player_data[player_id as usize] {
        // parent player ui node
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            })
            .insert(PlayerUI)
            .with_children(|player_ui| {
                let is_flipped = player_id as usize % 2 == 1;

                if is_flipped {
                    build_inner_ui(player_id, player_ui);
                    build_outer_ui(
                        player_id,
                        player_ui,
                        asset_server,
                        is_flipped,
                        &player_data.character,
                    );
                } else {
                    build_outer_ui(
                        player_id,
                        player_ui,
                        asset_server,
                        is_flipped,
                        &player_data.character,
                    );
                    build_inner_ui(player_id, player_ui);
                }
            });
    }
}

fn build_outer_ui(
    player_id: PlayerIDComponent,
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    is_flipped: bool,
    character: &CharacterType,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(65.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::ColumnReverse,
                padding: UiRect::all(Val::Percent(5.0)),
                ..default()
            },
            ..default()
        })
        .insert(PlayerOuterUI)
        .with_children(|outer_ui| {
            // first ability slot
            build_player_ability_slot_ui(
                player_id,
                0,
                outer_ui,
                asset_server,
                character,
                is_flipped,
            );

            // second ability slot
            build_player_ability_slot_ui(
                player_id,
                1,
                outer_ui,
                asset_server,
                character,
                is_flipped,
            );
        });
}

fn build_player_ability_slot_ui(
    player_id: PlayerIDComponent,
    ability_index: usize,
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    character: &CharacterType,
    is_flipped: bool,
) {
    let ability_slot_image = asset_server.load(if is_flipped {
        "texture/ability_square_right.png"
    } else {
        "texture/ability_square_left.png"
    });

    parent
        .spawn(ImageBundle {
            image: ability_slot_image.into(),
            style: Style {
                width: Val::Percent(100.0),
                aspect_ratio: Some(1.0),
                ..default()
            },
            ..default()
        })
        .insert(AbilitySlotUI { ability_index })
        .insert(player_id)
        .with_children(|ability_slot_ui| {
            ability_slot_ui
                .spawn(ImageBundle {
                    image: asset_server
                        .load(match ability_index {
                            0 => match character {
                                thetawave_interface::character::CharacterType::Captain => {
                                    "texture/blast_ability.png"
                                }
                                thetawave_interface::character::CharacterType::Juggernaut => {
                                    "texture/bullet_ability.png"
                                }
                            },
                            _ => match character {
                                thetawave_interface::character::CharacterType::Captain => {
                                    "texture/megablast_ability.png"
                                }
                                thetawave_interface::character::CharacterType::Juggernaut => {
                                    "texture/charge_ability.png"
                                }
                            },
                        })
                        .into(),
                    style: Style {
                        width: Val::Percent(100.0),
                        aspect_ratio: Some(1.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .insert(AbilityIconUI)
                .with_children(|ability_icon_ui| {
                    ability_icon_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::BLACK.with_a(0.85).into(),
                            ..default()
                        })
                        .insert(player_id)
                        .insert(AbilityValueUI { ability_index });
                });
        });
}

fn build_inner_ui(player_id: PlayerIDComponent, parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(35.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Percent(5.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            ..default()
        })
        .insert(PlayerInnerUI)
        .with_children(|inner_ui| {
            inner_ui
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
                .insert(HealthUI)
                .with_children(|health_ui| {
                    health_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::CRIMSON.with_a(0.75).into(),
                            ..default()
                        })
                        .insert(HealthValueUI)
                        .insert(player_id);
                });

            inner_ui
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
                .insert(ShieldsUI)
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
                        .insert(ShieldsValueUI)
                        .insert(player_id);
                });

            inner_ui
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
                .insert(ArmorUI)
                .insert(player_id);
        });
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
        .insert(ArmorCounterUI);
}

pub fn update_player_ui_system(
    mut commands: Commands,
    player_query: Query<
        (
            &HealthComponent,
            &PlayerIDComponent,
            &PlayerAbilitiesComponent,
            &WeaponComponent,
        ),
        (
            Changed<HealthComponent>,
            Changed<PlayerAbilitiesComponent>,
            Changed<WeaponComponent>,
        ),
    >,
    mut player_ui: ParamSet<(
        Query<(&mut Style, &PlayerIDComponent), With<HealthValueUI>>,
        Query<(&mut Style, &PlayerIDComponent), With<ShieldsValueUI>>,
        Query<(Entity, &PlayerIDComponent), With<ArmorUI>>,
        Query<(&mut Style, &AbilityValueUI, &PlayerIDComponent)>,
    )>,
) {
    for (player_health, player_id, player_abilities, weapon_component) in player_query.iter() {
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

        for (mut style, ability_value_ui, ability_id) in player_ui.p3().iter_mut() {
            if player_id == ability_id && ability_value_ui.ability_index == 0 {
                style.height =
                    Val::Percent(100.0 * (1.0 - weapon_component.reload_timer.fraction()));
            }
        }

        for (mut style, ability_value_ui, ability_id) in player_ui.p3().iter_mut() {
            if player_id == ability_id && ability_value_ui.ability_index == 1 {
                style.height = Val::Percent(
                    100.0 * (1.0 - player_abilities.ability_cooldown_timer.fraction()),
                );
            }
        }
    }
}
