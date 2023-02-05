use crate::{physics_sprite::PhysicsSpriteBundle, *};
use bevy::sprite::Mesh2dHandle;
use rand::Rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Default, EnumIter, Debug)]
pub enum PickupKind {
    #[default]
    MaxHealthUp,
    DamageUp,
    ShotSpeedUp,
    FireRateUp,
}

#[derive(Component, Default)]
pub struct Pickup {
    pub kind: PickupKind,
}

impl Pickup {
    pub fn apply(&self, stats: &mut player::Stats) {
        dbg!(&self.kind);
        match self.kind {
            PickupKind::DamageUp => stats.damage.multiply += 0.10,
            PickupKind::MaxHealthUp => stats.max_health.add += 20.0,
            PickupKind::ShotSpeedUp => stats.shot_speed.multiply += 0.1,
            PickupKind::FireRateUp => stats.fire_interval.multiply -= 0.1,
        }
    }
}

#[derive(Bundle, Default)]
pub struct PickupBundle {
    pickup: Pickup,
    #[bundle]
    sprite: PhysicsSpriteBundle,
}

impl PickupBundle {
    pub const PICKUP_DIMS: Vec2 = vec2(50.0, 50.0);

    pub fn new(tex: Handle<ColorMaterial>, mesh: Mesh2dHandle) -> PickupBundle {
        let r = rand::thread_rng().gen_range(0..4);
        let mut i = 0;
        let mut kind = PickupKind::default();
        for k in PickupKind::iter() {
            if i == r {
                kind = k;
                break;
            }
            i += 1;
        }

        return PickupBundle {
            pickup: Pickup { kind: kind },
            sprite: PhysicsSpriteBundle::new(&PickupBundle::PICKUP_DIMS, &Vec2::ZERO, tex, mesh),
        };
    }
}

pub fn tick(
    mut commands: Commands,
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut pickups: Query<(Entity, &mut Pickup, &mut Transform, &Collider)>,
    mut player: Query<(&mut Player, &mut Transform, &Collider, Without<Pickup>)>,
    rapier_ctx: Res<RapierContext>,
) {
    for (pickup_entity, mut pickup, mut transform, collider) in pickups.iter_mut() {
        rapier_ctx.intersections_with_shape(
            transform.translation.truncate(),
            0.0,
            collider,
            QueryFilter::default(),
            |entity| {
                if let Ok(mut player) = player.get_mut(entity) {
                    pickup.apply(&mut player.0.stats);
                    commands.entity(pickup_entity).despawn();
                }
                true
            },
        );
    }
}
