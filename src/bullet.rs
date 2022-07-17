use crate::*;

#[derive(Component)]
pub struct Bullet {
    pub shooter: Entity,
    pub hits_player: bool,
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Bullet {
    pub fn update_position(&mut self, delta: &Time) {
        self.position += self.velocity * delta.delta_seconds();
    }
}

pub fn tick(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(&mut Bullet, &mut Transform, &Collider)>,
    enemies: Query<(&mut Enemy, &mut Transform, &Collider, Without<Bullet>)>,
    rapier_ctx: Res<RapierContext>,
    mut game: ResMut<Game>,
) {
    for (mut bullet, mut transform, collider) in bullets.iter_mut() {
        bullet.update_position(time.as_ref());
        *transform = Transform {
            translation: Vec3::new(bullet.position.x, bullet.position.y, 32.0f32),
            ..default()
        };

        rapier_ctx.intersections_with_shape(
            transform.translation.truncate(),
            0.0,
            collider,
            QueryFilter::default(),
            |entity| {
                if let Ok(enemy) = enemies.get(entity) {
                    game.player.score += enemy.0.point_value;
                    commands.entity(entity).despawn();
                }
                true
            },
        );
    }
}
