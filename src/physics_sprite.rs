use bevy::sprite::Mesh2dHandle;

use crate::*;

#[derive(Component, Default)]
pub struct Dims(Vec2);

#[derive(Default, Bundle)]
pub struct PhysicsSpriteBundle {
    pub dims: Dims,
    pub collider: Collider,
    pub sensor: Sensor,

    //pub sprite: SpriteBundle,

    #[bundle]
    pub mesh: ColorMesh2dBundle,
}


impl PhysicsSpriteBundle {
    pub fn new(dims: &Vec2, pos: &Vec2, material: Handle<ColorMaterial>, mesh: Mesh2dHandle) -> PhysicsSpriteBundle {
        return PhysicsSpriteBundle {
            
            collider: Collider::capsule_y(dims.y / 4.0, dims.x / 4.0),
            sensor: Sensor,
            dims: Dims(dims.clone()),
            mesh: ColorMesh2dBundle {
                material: material,
                mesh: mesh,
                transform: Transform::from_translation(pos.extend(0.0)),
                ..Default::default()
            },
        };
    }
}
