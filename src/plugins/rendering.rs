use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::{
    components::*,
    states::*,
};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlayState::Setup), (
                setup_skybox,
                setup_lighting,
            ))
            .add_systems(Update, (
                reinterpret_cubemap,
                update_light_position,
            ).run_if(in_state(GameState::Playing)));
    }
}

fn setup_skybox(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let sky_image = asset_server.load("skysheet.png");

    info!("Loading skybox texture: skysheet.png");

    commands.insert_resource(SkyCubeMap {
        image: sky_image,
        loaded: false,
    });
}

fn setup_lighting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.05,
    });

    // Lamp posts
    let lamp_positions = vec![
        Vec3::new(-5.0, 0.0, 3.0),
        Vec3::new(5.0, 0.0, 3.0),
        Vec3::new(0.0, 0.0, 4.0),
    ];

    for (i, position) in lamp_positions.iter().enumerate() {
        let lamp_entity = commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cylinder {
                    radius: 0.1,
                    height: 4.0,
                    resolution: 8,
                    segments: 1,
                })),
                material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
                transform: Transform::from_translation(*position),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cylinder(2.0, 0.1),
            Name::new(format!("LampPost_{}", i)),
        )).id();

        // Add light
        let light_entity = commands.spawn((
            PointLightBundle {
                point_light: PointLight {
                    intensity: 2000.0,
                    shadows_enabled: true,
                    color: Color::rgb(1.0, 0.9, 0.7),
                    range: 20.0,
                    radius: 0.5,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 2.0, 0.0),
                ..default()
            },
            Name::new(format!("LampLight_{}", i)),
        )).id();

        commands.entity(lamp_entity).push_children(&[light_entity]);
    }

    // Follow light
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 500.0,
                shadows_enabled: false,
                color: Color::rgb(0.8, 0.8, 1.0),
                range: 10.0,
                radius: 0.3,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 2.0, 2.0),
            ..default()
        },
        FollowLight {
            offset: Vec3::new(0.0, 2.0, 2.0),
        },
        Name::new("FollowingLight"),
    ));
}

fn reinterpret_cubemap(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<SkyCubeMap>,
    mut commands: Commands,
    mut skyboxes: Query<&mut bevy::core_pipeline::Skybox>,
    camera_query: Query<Entity, (With<Camera3d>, With<FollowCamera>, Without<bevy::core_pipeline::Skybox>)>,
) {
    if !cubemap.loaded && asset_server.load_state(&cubemap.image) == bevy::asset::LoadState::Loaded {
        cubemap.loaded = true;
        let image = images.get_mut(&cubemap.image).unwrap();

        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.sampler = bevy::render::texture::ImageSampler::Descriptor(
                bevy::render::texture::ImageSamplerDescriptor::nearest()
            );
            image.texture_view_descriptor = Some(bevy::render::render_resource::TextureViewDescriptor {
                dimension: Some(bevy::render::render_resource::TextureViewDimension::Cube),
                ..Default::default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.0 = cubemap.image.clone();
        }

        if let Ok(camera_entity) = camera_query.get_single() {
            commands.entity(camera_entity).insert(bevy::core_pipeline::Skybox(cubemap.image.clone()));
            info!("Skybox added to camera");
        }
    }
}

fn update_light_position(
    boss_query: Query<&Transform, With<Boss>>,
    mut light_query: Query<(&mut Transform, &FollowLight), Without<Boss>>,
) {
    if let Ok(boss_transform) = boss_query.get_single() {
        for (mut light_transform, follow_light) in light_query.iter_mut() {
            light_transform.translation = boss_transform.translation + follow_light.offset;
        }
    }
}