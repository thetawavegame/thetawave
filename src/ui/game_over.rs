use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

use crate::{
    states::{AppStateComponent, AppStates},
    ui::BouncingPromptComponent,
};

#[derive(Component)]
pub struct GameFadeComponent;
#[derive(Component)]
pub struct GameOverFadeComponent;

pub struct EndGameTransitionResource {
    pub fade_out_timer: Timer,
    pub fade_in_timer: Timer,
    pub max_fps: f32,
    pub frame_slowdown_speed: f32,
    pub start: bool,
    pub fade_out_speed: f32,
    pub fade_in_speed: f32,
    pub next_state: Option<AppStates>,
}

impl EndGameTransitionResource {
    pub fn new(
        fade_out_seconds: f32,
        fade_in_seconds: f32,
        frame_slowdown_speed: f32,
        fade_out_speed: f32,
        fade_in_speed: f32,
        max_fps: f32,
    ) -> Self {
        Self {
            fade_out_timer: Timer::from_seconds(fade_out_seconds, false),
            fade_in_timer: Timer::from_seconds(fade_in_seconds, false),
            start: false,
            max_fps,
            frame_slowdown_speed,
            fade_out_speed,
            fade_in_speed,
            next_state: None,
        }
    }

    pub fn start(&mut self, app_state: AppStates) {
        self.start = true;
        self.next_state = Some(app_state)
    }
}

#[derive(Component)]
pub struct GameOverUI;

#[cfg(not(target_arch = "wasm32"))]
pub fn fade_out_system(
    mut app_state: ResMut<State<AppStates>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut framepace: ResMut<bevy_framepace::FramepacePlugin>,
    time: Res<Time>,
    mut end_game_trans_resource: ResMut<EndGameTransitionResource>,
    mut game_fade_query: Query<&mut Sprite, With<GameFadeComponent>>,
) {
    if end_game_trans_resource.start {
        end_game_trans_resource.fade_out_timer.tick(time.delta());

        let framerate: u16 = ((-((end_game_trans_resource.frame_slowdown_speed
            * end_game_trans_resource.fade_out_timer.elapsed_secs())
        .powf(2.0))
            + end_game_trans_resource.max_fps) as u16)
            .max(1);

        use bevy_framepace::FramerateLimit;
        framepace.framerate_limit = FramerateLimit::Manual(framerate);

        for mut fade_sprite in game_fade_query.iter_mut() {
            let alpha = (end_game_trans_resource.fade_out_speed
                * end_game_trans_resource.fade_out_timer.elapsed_secs())
            .min(1.0);

            fade_sprite.color.set_a(alpha);
        }

        if end_game_trans_resource.fade_out_timer.just_finished() {
            rapier_config.physics_pipeline_active = false;
            rapier_config.query_pipeline_active = false;
            framepace.framerate_limit = FramerateLimit::Auto;
            app_state
                .set(end_game_trans_resource.next_state.as_ref().unwrap().clone())
                .unwrap();
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn fade_out_system(
    mut app_state: ResMut<State<AppStates>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    time: Res<Time>,
    mut end_game_trans_resource: ResMut<EndGameTransitionResource>,
    mut game_fade_query: Query<&mut Sprite, With<GameFadeComponent>>,
) {
    if end_game_trans_resource.start {
        end_game_trans_resource.fade_out_timer.tick(time.delta());

        for mut fade_sprite in game_fade_query.iter_mut() {
            let alpha = (end_game_trans_resource.fade_out_speed
                * end_game_trans_resource.fade_out_timer.elapsed_secs())
            .min(1.0);

            fade_sprite.color.set_a(alpha);
        }

        if end_game_trans_resource.fade_out_timer.just_finished() {
            rapier_config.physics_pipeline_active = false;
            rapier_config.query_pipeline_active = false;
            app_state
                .set(end_game_trans_resource.next_state.as_ref().unwrap().clone())
                .unwrap();
        }
    }
}

pub fn game_over_fade_in_system(
    time: Res<Time>,
    mut end_game_trans_resource: ResMut<EndGameTransitionResource>,
    mut game_over_fade_query: Query<&mut UiColor, With<GameOverFadeComponent>>,
) {
    end_game_trans_resource.fade_in_timer.tick(time.delta());

    let timer_finished = end_game_trans_resource.fade_in_timer.finished();

    for mut color in game_over_fade_query.iter_mut() {
        if !timer_finished {
            let alpha = (end_game_trans_resource.fade_in_speed
                * end_game_trans_resource.fade_in_timer.elapsed_secs())
            .min(1.0);

            color.0.set_a(alpha);
        } else {
            color.0.set_a(1.0);
        }
    }
}

pub fn setup_game_over_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::GameOver))
        .insert(GameOverUI)
        .with_children(|parent| {
            parent
                .spawn_bundle(ImageBundle {
                    image: asset_server.load("texture/game_over_background.png").into(), // not using assetsmanager as we don't load everything on the main menu
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::rgba(1.0, 1.0, 1.0, 0.0).into(),
                    ..default()
                })
                .insert(GameOverFadeComponent)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ImageBundle {
                            image: asset_server
                                .load("texture/restart_game_prompt_keyboard.png")
                                .into(),
                            style: Style {
                                size: Size::new(Val::Px(400.0), Val::Px(100.0)),
                                margin: Rect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    top: Val::Percent(20.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(BouncingPromptComponent {
                            flash_timer: Timer::from_seconds(2.0, true),
                        });
                    parent
                        .spawn_bundle(ImageBundle {
                            image: asset_server
                                .load("texture/exit_game_prompt_keyboard.png")
                                .into(),
                            style: Style {
                                size: Size::new(Val::Px(400.0), Val::Px(100.0)),
                                margin: Rect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    top: Val::Percent(20.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(BouncingPromptComponent {
                            flash_timer: Timer::from_seconds(2.0, true),
                        });
                });
        });
}
