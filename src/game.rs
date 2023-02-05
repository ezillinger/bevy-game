use crate::*;

pub fn spawn_waves(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut colors: ResMut<Assets<ColorMaterial>>,
    enemies: Query<(Entity, With<Enemy>)>,
    pickups: Query<(Entity, (With<Pickup>, Without<Enemy>))>,
) {
    if enemies.is_empty() && pickups.is_empty() {
        if game.wave % 5 == 0 {
            commands.spawn(PickupBundle::new(
                colors.add(ColorMaterial {
                    color: Color::rgba(1.0, 0.0, 0.0, 1.0).into(),
                    texture: None,
                }),
                game.handles.pickup_mesh.clone(),
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
