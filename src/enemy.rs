use crate::*;

#[derive(Component, Default)]
pub struct Enemy {
    pub position: Vec2,
    pub radius: f32,
}

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    enemy: Enemy,
    collider: Collider,
    sensor: Sensor,

    #[bundle]
    sprite: SpriteBundle,
}

impl EnemyBundle {
    pub fn new(pos: Vec2, tex: Handle<Image>) -> Self {
        return Self {
            enemy: Enemy {
                position: pos,
                radius: 1.0,
            },
            collider: Collider::cuboid(100.0, 100.0),
            sprite: SpriteBundle {
                texture: tex,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(200.0, 220.0)),
                    ..default()
                },
                ..default()
            },
            ..default()
        };
    }
}

pub fn tick(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    time: Res<Time>,
    mut enemies: Query<(&mut Enemy, &mut Transform)>,
) {
    for (mut enemy, mut transform) in enemies.iter_mut() {
        enemy.position += rand_vec2() / 100.0f32;
        *transform = Transform {
            translation: Vec3::new(enemy.position.x, enemy.position.y, 32.0f32),
            ..default()
        };
    }
}
