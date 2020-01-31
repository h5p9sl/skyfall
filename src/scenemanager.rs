use crate::gamestate::GameState;
use ::backend::{RectangleShape, shapes, Camera, RenderWindow};
use ::input::InputManager;
use piston::{keyboard::Key, Button, Input};

use crate::local_player::LocalPlayer;

pub struct SceneManager {
    bg_color: [f32; 4],
    local_player: LocalPlayer,
    window_size: shapes::Size,
    ground: RectangleShape,
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            bg_color: [0.40, 0.14, 0.16, 1.0],
            local_player: LocalPlayer::new(),
            window_size: shapes::Size::from([0., 0.]),
            ground: RectangleShape::new().size([800.0, 400.0]).position([0.0, 0.0]),
        }
    }

    pub fn resize(&mut self, _size: shapes::Size) {}

    pub fn on_input(&mut self, input: &Input) -> GameState {
        match input {
            Input::Button(args) => {
                if args.button == Button::Keyboard(Key::Escape) {
                    return GameState::Paused;
                }
                if args.button == Button::Keyboard(Key::F2) {
                    if args.state == piston::ButtonState::Press {
                        self.local_player.toggle_debug();
                    }
                }
            }
            _ => {}
        }
        GameState::InGame
    }

    pub fn update_camera(&mut self, input: &InputManager, camera: &mut Camera) {
        // Half way through the window width
        let window_width = self.window_size.w;
        // Get cursor position -width -> 0.0
        let mouse_pos = shapes::Point::from(input.get_cursor_pos()) - window_width;
        let mut player_pos = self.local_player.get_position();
        // Offset player position by cursor position
        player_pos.x = -player_pos.x - mouse_pos.x;
        player_pos.y = -player_pos.y - mouse_pos.y;

        camera.set_position(player_pos);
    }

    pub fn update(&mut self, delta: f64, input: &InputManager, camera: &mut Camera) {
        self.local_player.update(delta, input, camera);
        self.update_camera(input, camera);
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        window.clear(self.bg_color);
        window.draw(&mut self.ground);
        self.local_player.draw(window);
        self.window_size = window.size();
    }
}
