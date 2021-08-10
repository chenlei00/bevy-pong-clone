use crate::{Collider, Player};
use bevy::app::EventReader;
use bevy::core::Time;
use bevy::ecs::system::{Commands, Query, Res};
use bevy::input::keyboard::KeyCode;
use bevy::input::Input;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::Sprite;
use bevy::sprite::entity::SpriteBundle;
use bevy::transform::components::Transform;
use bevy::window::WindowResized;

#[derive(Default)]
pub struct Paddle {
	speed: f32,
}

impl Paddle {
	const WIDTH: f32 = 20.0;
	const MARGIN: f32 = 50.0;
}

pub fn paddle_resize_system(
	mut resize_reader: EventReader<WindowResized>,
	mut paddle_query: Query<(&mut Sprite, &mut Transform, &mut Paddle, &Player)>,
) {
	let resize_event = match resize_reader.iter().last() {
		Some(event) => event,
		None => return,
	};

	for (mut sprite, mut transform, mut paddle, &player) in paddle_query.iter_mut() {
		let window_height = resize_event.height as f32;
		let window_width = resize_event.width as f32;
		paddle.speed = (window_height as f32) / 3.0;

		sprite.size = Vec2::new(Paddle::WIDTH, 0.2 * window_height);

		use Player::*;
		let x_translation = match player {
			Left => Paddle::MARGIN - (window_width / 2.0),
			Right => (window_width / 2.0) - Paddle::MARGIN,
		};

		transform.translation = Vec3::new(x_translation, 0.0, 0.0);
	}
}

pub fn spawn_paddles(commands: &mut Commands) {
	spawn_paddle(commands, Player::Left);
	spawn_paddle(commands, Player::Right);
}

fn spawn_paddle(commands: &mut Commands, player: Player) {
	commands
		.spawn()
		.insert_bundle(SpriteBundle::default())
		.insert(Paddle::default())
		.insert(player)
		.insert(Collider);
}

pub fn paddle_movement_system(
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&Paddle, &Player, &mut Transform)>,
) {
	let time_delta = time.delta_seconds();

	for (paddle, player, mut transform) in query.iter_mut() {
		let (up_keycode, down_keycode) = player.movement_keys();

		if keyboard_input.pressed(up_keycode) {
			transform.translation += time_delta * Vec2::new(0.0, paddle.speed).extend(0.0);
		}

		if keyboard_input.pressed(down_keycode) {
			transform.translation += time_delta * Vec2::new(0.0, -paddle.speed).extend(0.0);
		}
	}
}
