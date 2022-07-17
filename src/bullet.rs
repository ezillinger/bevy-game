use crate::*;

#[derive(Component)]
pub struct Bullet {
    pub shooter: Entity,
    pub hits_player: bool,
    pub position: Vec2,
    pub velocity: Vec2,
    pub damage: i32,
}

impl Bullet {
    pub fn update_position(&mut self, delta: &Time) {
        self.position += self.velocity * delta.delta_seconds();
    }
}

pub fn tick(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Bullet, &mut Transform, &Collider)>,
    mut enemies: Query<(&mut Enemy, &mut Transform, &Collider, Without<Bullet>)>,
    rapier_ctx: Res<RapierContext>,
    game: ResMut<Game>,
) {
    for (bullet_entity, mut bullet, mut transform, collider) in bullets.iter_mut() {
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
                if let Ok(mut enemy) = enemies.get_mut(entity) {
                    enemy.0.health -= bullet.damage;
                    commands.entity(bullet_entity).despawn();
                }
                true
            },
        );
    }
}
