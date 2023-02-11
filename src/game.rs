use crate::*;
use rand::Rng;
use strum::{IntoEnumIterator, EnumCount};

pub fn spawn_waves(
    mut commands: Commands,
    mut game: ResMut<Game>,
    colors: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    enemies: Query<(Entity, With<Enemy>)>,
    pickups: Query<(Entity, (With<Pickup>, Without<Enemy>))>,
) {
    if enemies.is_empty() && pickups.is_empty() {
        if game.wave % 2 == 0 {
            let r = rand::thread_rng().gen_range(0..PickupKind::COUNT);
            let mut i = 0;
            let mut kind = PickupKind::default();
            for k in PickupKind::iter() {
                if i == r {
                    kind = k;
                    break;
                }
                i += 1;
            };
            commands.spawn(PickupBundle::from_kind(
                Vec2::ZERO, kind, colors, meshes
            ));
        } else {
            for _ in 0..(game.wave + 5) {
                commands.spawn(EnemyBundle::new(
                    rand_norm_vec2() * MAP_DIMS / 2.0,
                    game.handles.enemy_atlas.clone(),
                ));
            }
        }
        game.wave += 1;
    }
}
