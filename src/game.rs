use crate::*;

pub fn spawn_waves(
    mut commands: Commands,
    mut game: ResMut<Game>,
    enemies: Query<(Entity, With<Enemy>)>,
    pickups: Query<(Entity, (With<Pickup>, Without<Enemy>))>,
) {
    if enemies.is_empty() && pickups.is_empty() {
        if game.wave % 5 == 0 {
            commands.spawn_bundle(PickupBundle::new(game.handles.pickup_tex.clone()));
        } else {
            for _ in 0..(game.wave + 5) {
                commands.spawn_bundle(EnemyBundle::new(
                    rand_norm_vec2() * MAP_DIMS / 2.0,
                    game.handles.enemy_atlas.clone(),
                    game.handles.enemy_animation.clone(),
                ));
            }
        }
        game.wave += 1;
    }
}
