use crate::{physics_sprite::PhysicsSpriteBundle, *};

#[derive(Component, Default)]
pub struct Bullet {
    pub shooter: Option<Entity>,
    pub hits_player: bool,
    pub position: Vec2,
    pub velocity: Vec2,
    pub damage: f32,
    pub radius: f32,

    pub piercing: i32,
    pub hit_enemies: Vec<Entity>,
}

impl Bullet {
    pub fn update_position(&mut self, delta: &Time) {
        self.position += self.velocity * delta.delta_seconds();
    }
}

#[derive(Default, Bundle)]
pub struct BulletBundle {
    bullet: Bullet,
    #[bundle]
    sprite: PhysicsSpriteBundle,
}

impl BulletBundle {
    pub fn new(bullet: Bullet, mesh: Mesh2dHandle) -> Self {
        BulletBundle {
            sprite: PhysicsSpriteBundle {
                collider: Collider::ball(bullet.radius),
                sensor: Sensor,
                mesh: ColorMesh2dBundle {
                    transform: Transform {
                        translation: bullet.position.extend(32.0f32),
                        ..default()
                    },
                    mesh: mesh,
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

                    if let None = bullet.hit_enemies.iter().find(|&&x| x == entity){

                        enemy.0.direction = bullet.velocity.normalize();
                        enemy.0.health -= bullet.damage;
                        if enemy.0.health <= 0.0 {
                            game.kills += 1;
                            game.player.score += enemy.0.max_health;
                        }

                        bullet.piercing -= 1;
                        if bullet.piercing <= 0 {
                            commands.entity(bullet_entity).despawn();
                        }
                        else{
                            bullet.hit_enemies.push(entity)
                        }
                    }
                }
                true
            },
        );
    }
}
