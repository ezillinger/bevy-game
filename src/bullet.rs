use crate::*;

#[derive(Component)]
pub struct Bullet {
    pub shooter: Entity,
    pub hits_player: bool,
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Bullet {
    pub fn update_position(&mut self, delta: &Time){
        self.position += self.velocity * delta.delta_seconds();
    }
}

pub fn tick_bullets(mut commands : Commands, input: Res<Input<KeyCode>>, mut game: ResMut<Game>, time: Res<Time>, mut bullets: Query<(&mut Bullet, &mut Transform)>){
    for (mut bullet, mut transform) in bullets.iter_mut() {
        bullet.update_position(time.as_ref());
        *transform = Transform{ translation: Vec3::new(bullet.position.x, bullet.position.y, 32.0f32), ..default() };
    }

}