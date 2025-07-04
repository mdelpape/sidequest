use bevy::{
    asset::LoadState,
    core_pipeline::Skybox,
    prelude::*,
    render::{
        render_resource::{TextureViewDescriptor, TextureViewDimension},
        texture::{ImageSampler, ImageSamplerDescriptor},
    },
};
use crate::components::{SkyCubeMap, FollowCamera};

pub fn setup_skybox(
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

pub fn reinterpret_cubemap(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<SkyCubeMap>,
    mut commands: Commands,
    mut skyboxes: Query<&mut Skybox>,
    camera_query: Query<Entity, (With<Camera3d>, With<FollowCamera>, Without<Skybox>)>,
) {
    if !cubemap.loaded && asset_server.load_state(&cubemap.image) == LoadState::Loaded {
        cubemap.loaded = true;
        let image = images.get_mut(&cubemap.image).unwrap();

        if image.texture_descriptor.array_layer_count() == 1 {
            // Convert 2D texture to cube map array
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::nearest());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..Default::default()
            });
        }

        // Set all skybox images to the new array texture
        for mut skybox in &mut skyboxes {
            skybox.0 = cubemap.image.clone();
        }

        // Add skybox component to camera if it doesn't have one yet
        if let Ok(camera_entity) = camera_query.get_single() {
            commands.entity(camera_entity).insert(Skybox(cubemap.image.clone()));
            info!("Skybox added to camera");
        }
    }
}