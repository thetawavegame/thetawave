use bevy::prelude::*;

#[derive(Component)]
pub struct GameOverUI;

pub fn setup_game_over_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GameOverUI)
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                image: asset_server.load("texture/game_over_background.png").into(), // not using assetsmanager as we don't load everything on the main menu
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..default()
            });
        });
}

pub fn clear_game_over_system(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<GameOverUI>>,
) {
    for entity in main_menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
