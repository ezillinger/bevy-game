use crate::*;

#[derive(Component, Default)]
pub struct Bullet {
    pub shooter: Option<Entity>,
    pub hits_player: bool,
    pub position: Vec2,
    pub velocity: Vec2,
    pub damage: f32,
    pub radius: f32,
}

impl Bullet {
    pub fn update_position(&mut self, delta: &Time) {
        self.position += self.velocity * delta.delta_seconds();
    }
}

#[derive(Default, Bundle)]
pub struct BulletBundle {
    bullet: Bullet,
    collider: Collider,
    sensor: Sensor,
    #[bundle]
    sprite: SpriteBundle,
}

impl BulletBundle {
    pub fn new(bullet: Bullet) -> Self {
        BulletBundle {
            collider: Collider::ball(bullet.radius),
            sensor: Sensor,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::ONE * 2.0 * bullet.radius),
                    ..default()
                },
                transform: Transform {
                    translation: bullet.position.extend(32.0f32),
                    ..default()
                },
                ..default()
            },
            bullet: bullet,
        }
    }
}

pub fn tick(
    mut commands: Commands,
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut bullets: Query<(Entity, &mut Bullet, &mut Transform, &Collider)>,
    mut enemies: Query<(&mut Enemy, &mut Transform, &Collider, Without<Bullet>)>,
    rapier_ctx: Res<RapierContext>,
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
                    enemy.0.direction = bullet.velocity.normalize();
                    commands.entity(bullet_entity).despawn();
                    game.kills += 1;
                    game.player.score += enemy.0.max_health;
                }
                true
            },
        );
    }
}
