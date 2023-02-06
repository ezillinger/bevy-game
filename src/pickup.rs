use crate::{physics_sprite::PhysicsSpriteBundle, *};
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use player::Player;
use rand::Rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Default, EnumIter, Debug, Clone)]
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
    pub fn apply(&self, player: &mut Player) {
        dbg!(&self.kind);
        match self.kind {
            PickupKind::DamageUp => player.stats.damage.multiply += 0.10,
            PickupKind::MaxHealthUp => {
                player.stats.max_health.add += 20.0;
                player.health += 20.0;
            }
            PickupKind::ShotSpeedUp => player.stats.shot_speed.multiply += 0.1,
            PickupKind::FireRateUp => player.stats.fire_interval.multiply -= 0.1,
        }
    }

    pub fn get_color(kind: PickupKind) -> Color {
        match kind {
            PickupKind::MaxHealthUp => Color::RED,
            PickupKind::DamageUp => Color::ORANGE,
            PickupKind::FireRateUp => Color::GREEN,
            PickupKind::ShotSpeedUp => Color::LIME_GREEN,
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

    pub fn from_kind(
        pos: Vec2,
        kind: PickupKind,
        mut colors: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) -> PickupBundle {
        let dims = vec2(40.0, 40.0);
        let mesh: Mesh = shape::Box::new(dims.x, dims.y, 1.0).into();
        PickupBundle {
            pickup: Pickup { kind: kind.clone() },
            sprite: PhysicsSpriteBundle::new(
                &dims,
                &pos,
                colors.add(Pickup::get_color(kind).into()),
                meshes.add(mesh).into(),
            ),
        }
    }
}

pub fn tick(
    mut commands: Commands,
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut pickups: Query<(Entity, &mut Pickup, &mut Transform, &Collider, &mut Handle<ColorMaterial>)>,
    mut player: Query<(&mut Player, &mut Transform, &Collider, Without<Pickup>)>,
    rapier_ctx: Res<RapierContext>,
) {
    for (pickup_entity, pickup, transform, collider, material) in pickups.iter_mut() {
        let osc = f32::sqrt(((time.elapsed_seconds_wrapped() * 5.0).sin() + 1.0)/ 2.0);
        let color = Pickup::get_color(pickup.kind.clone()).as_hsla_f32();
        let brightness = color[2] * (1.0 + 0.9 * osc);
        materials.get_mut(&material).expect("no material").color = Color::hsla(color[0], color[1], brightness, color[3]); 
        rapier_ctx.intersections_with_shape(
            transform.translation.truncate(),
            0.0,
            collider,
            QueryFilter::default(),
            |entity| {
                if let Ok(mut player) = player.get_mut(entity) {
                    pickup.apply(&mut player.0);
                    pickup.apply(&mut game.player);
                    commands.entity(pickup_entity).despawn();
                }
                true
            },
        );
    }
}
