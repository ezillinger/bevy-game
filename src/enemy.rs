use crate::*;

#[derive(Component)]
pub struct Enemy {
    pub position: Vec2,
    pub radius: f32
}

impl Enemy {

    pub fn move_to(&mut self, pos: Vec2) {
        self.position = pos;
    }

}

pub fn tick_enemies(mut commands : Commands, input: Res<Input<KeyCode>>, mut game: ResMut<Game>, time: Res<Time>, mut enemies: Query<(&mut Enemy, &mut Transform)>){
    for (mut enemy, mut transform) in enemies.iter_mut(){
        enemy.position += rand_vec2() / 100.0f32;
        *transform = Transform{ translation: Vec3::new(enemy.position.x, enemy.position.y, 32.0f32), ..default() };
    }

}