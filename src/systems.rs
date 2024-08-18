use bevy::{prelude::*, render::texture::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor}};

pub fn spawn_camera(
    mut commands: Commands
)
{
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }
    );
}

pub fn set_image_meta(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut assets: ResMut<Assets<Image>>,
) {
    for ev in ev_asset.read() {
        match ev {
            AssetEvent::LoadedWithDependencies { id } => {
                if let Some(texture) = assets.get_mut(*id) {
                    texture.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        ..default()
                    })
                }
            },
            _ => {}
        }
    }
}