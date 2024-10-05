use crate::spawnable::SpawnableBehavior;
use crate::{
    animation::AnimationComponent, game::GameParametersResource, spawnable::SpawnableComponent,
};
use bevy::prelude::{
    in_state, App, Commands, EventReader, IntoSystemConfigs, Name, Plugin, Res, Timer, TimerMode,
    Transform, Update, Vec2, Vec3,
};
use bevy::sprite::{SpriteBundle, TextureAtlas};
use bevy::utils::default;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, LockedAxes, RigidBody, Sensor, Velocity};
use thetawave_assets::ItemAssets;
use thetawave_interface::spawnable::AttractToClosestPlayerComponent;
use thetawave_interface::spawnable::{ItemComponent, SpawnItemEvent};
use thetawave_interface::{
    spawnable::ItemType,
    states::{self, GameCleanup},
};

use super::{
    behavior::{ItemBehavior, OnCollectFullHeal, OnCollectIncreaseMaxHealth},
    ItemResource,
};

pub struct ItemSpawnPlugin;

impl Plugin for ItemSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_item_system
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing)),
        );
    }
}

pub fn spawn_item_system(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnItemEvent>,
    item_resource: Res<ItemResource>,
    item_assets: Res<ItemAssets>,
    game_parameters: Res<GameParametersResource>,
) {
    for event in event_reader.read() {
        spawn_item(
            &mut commands,
            &item_resource,
            &item_assets,
            &event.item_type,
            event.position,
            &game_parameters,
        );
    }
}

pub fn spawn_item(
    commands: &mut Commands,
    item_resource: &ItemResource,
    item_assets: &ItemAssets,
    item_type: &ItemType,
    position: Vec2,
    game_parameters: &GameParametersResource,
) {
    //Get data from the item resource
    let item_data = &item_resource.items[item_type];

    // Scale collider to align with the sprite
    let collider_size_hx = item_data.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
    let collider_size_hy = item_data.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

    // Create item entity
    let mut item = commands.spawn_empty();

    if item_data
        .spawnable_behaviors
        .contains(&SpawnableBehavior::AttractToPlayer)
    {
        item.insert(AttractToClosestPlayerComponent);
    }

    // Sprite components
    item.insert(SpriteBundle {
        texture: item_assets.get_image(item_type),
        ..default()
    })
    .insert(TextureAtlas {
        layout: item_assets.get_texture_atlas_layout(item_type),
        ..default()
    })
    .insert(AnimationComponent {
        timer: Timer::from_seconds(item_data.animation.frame_duration, TimerMode::Repeating),
        direction: item_data.animation.direction.clone(),
    });

    // Movement components
    item.insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::from(item_data.initial_motion.clone()));

    // Position components
    item.insert(Transform {
        translation: position.extend(item_data.z_level),
        scale: Vec3::new(
            game_parameters.sprite_scale,
            game_parameters.sprite_scale,
            1.0,
        ),
        ..Default::default()
    });

    // Collider components
    item.insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(collider_size_hx, collider_size_hy))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS);

    // Core components
    item.insert(ItemComponent::from(item_data))
        .insert(SpawnableComponent::from(item_data));

    item.insert(GameCleanup);

    item.insert(Name::new(item_data.item_type.to_string()));

    // https://github.com/bevyengine/bevy/issues/3227
    add_item_behavior_components(item_data, item);
}

fn add_item_behavior_components(
    item_data: &super::ItemData,
    mut item: bevy::ecs::system::EntityCommands<'_>,
) {
    for behavior in item_data.item_behaviors.iter() {
        match behavior {
            ItemBehavior::OnCollectIncreaseMaxHealth(v) => {
                item.insert(OnCollectIncreaseMaxHealth(*v));
            }
            ItemBehavior::OnCollectFullHeal => {
                item.insert(OnCollectFullHeal);
            }
        };
    }
}
