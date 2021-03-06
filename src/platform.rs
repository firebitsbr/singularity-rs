use amethyst::{
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, World, WorldExt},
    prelude::*,
    renderer::{palette::Srgba, resources::Tint, SpriteRender},
};
use derivative::Derivative;

#[derive(Debug, Derivative, Copy, Clone)]
#[derivative(Default)]
pub enum PlatformType {
    #[derivative(Default)]
    Blank,
    // // As soon as there is 'Production' capability:
    // Well,
    // Mine,
    // Quarry,
    //
    // // As soon as there is PlatformActions:
    // Factory,
    // Junkyard,
    // Energy,
    //
    // // When creation of FreeMovingUnits is possible:
    // Barracks,
    // Command,
    //
    // // When there is fighting:
    // Kinetic,
    // Laser,
    //
    // // Other, more complex:
    // Packaging,
    // Storage,
    _ExhaustiveMatches,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct PlatformAttributes(PlatformType);

pub fn create_platform(
    platform: PlatformAttributes,
    world: &mut World,
    sprite_render: SpriteRender,
    x: f32,
    y: f32,
) {

    let mut trans = Transform::default();
    trans.set_translation_xyz(x, y, crate::layers::BasePlatformLayer);
    *trans.scale_mut() *= 0.25;

    let color = match platform.0 {
        PlatformType::Blank => Srgba::new(1.0, 1.0, 1.0, 0.8),
        _ => panic!("Unimplemented!"),
    };

    let tint = Tint(color);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(platform)
        .with(trans)
        .with(tint)
        .build();
}

impl Component for PlatformAttributes {
    type Storage = DenseVecStorage<Self>;
}

