use bevy::prelude::*;
use crate::{
    components::*,
    states::*,
    systems::lighting::setup_lighting,
};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlayState::Setup), (
                // setup_skybox,
                setup_lighting,
            ));

            // Temporarily disabled follow light and skybox
            // .add_systems(Update, (
            //     reinterpret_cubemap,
            //     // update_light_position,
            // ).run_if(in_state(GameState::Playing)));
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

