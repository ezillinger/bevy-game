use crate::*;

#[derive(Component, Default)]
pub struct Dims(Vec2);

#[derive(Default, Bundle)]
pub struct PhysicsSpriteBundle {
    pub dims: Dims,
    pub collider: Collider,
    pub sensor: Sensor,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl PhysicsSpriteBundle {
    pub fn new(dims: &Vec2, pos: &Vec2, tex: Handle<Image>) -> PhysicsSpriteBundle {
        return PhysicsSpriteBundle {
            collider: Collider::capsule_y(dims.y / 4.0, dims.x / 4.0),
            sensor: Sensor,
            sprite: SpriteBundle {
                texture: tex,
                sprite: Sprite {
                    custom_size: Some(dims.clone()),
                    ..default()
                },
                ..default()
            },
            dims: Dims(dims.clone()),
        };
    }
}
