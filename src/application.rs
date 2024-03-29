use crate::{config::Config, vec2::Vec2};
use std::time::Instant;

use device_query::{DeviceQuery, DeviceState, Keycode};
use mouse_rs::{types::keys::Keys, Mouse};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct Application {
    application_on: bool,
    motions_on: bool,
    mouse: Mouse,
    count: i32,
    delta_time: f64,
    last_tick: Instant,
    right_pressed: bool,
    left_pressed: bool,
    i_pressed: bool,
    mouse_pos: Vec2<f64>,
    config: Config,
}

impl Application {
    pub fn new() -> Self {
        let mouse = Mouse::new();
        let mouse_pos = mouse.get_position().unwrap();
        return Application {
            application_on: true,
            motions_on: true,
            mouse,
            count: 6,
            delta_time: 0.0,
            last_tick: std::time::Instant::now(),
            right_pressed: false,
            left_pressed: false,
            i_pressed: false,
            mouse_pos: Vec2 {
                x: mouse_pos.x as f64,
                y: mouse_pos.y as f64,
            },
            config: Config::from_config_file("./config.json"),
        };
    }

    pub fn run(mut self) {
        let device_state = DeviceState::new();
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(0.0, 0.0))
            .build(&event_loop)
            .unwrap();

        let mut toggle_key_pressed = false;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            std::thread::sleep(std::time::Duration::from_millis(1));

            let pressed_keys = device_state.get_keys();

            if pressed_keys.contains(&self.config.keybinds["toggle_key_modifier"])
                && pressed_keys.contains(&self.config.keybinds["toggle_key"])
            {
                if !toggle_key_pressed {
                    self.application_on = !self.application_on;
                    toggle_key_pressed = true;
                }
            } else {
                toggle_key_pressed = false;
            }

            if self.application_on {
                if pressed_keys.contains(&self.config.keybinds["motions_on_key"]) {
                    window.focus_window();
                    self.motions_on = true;
                }

                if self.motions_on {
                    if pressed_keys.contains(&self.config.keybinds["motions_off_key"]) {
                        // unfocus window
                        if !self.i_pressed {
                            self.mouse.click(&Keys::LEFT).unwrap();
                            self.motions_on = false;
                            self.i_pressed = true;
                        }
                    } else {
                        self.i_pressed = false;
                        self.handle_set_count(&pressed_keys);
                        self.handle_basic_motions(&pressed_keys);
                        self.handle_mouse_clicks(&pressed_keys);
                        self.handle_scrolling(&pressed_keys);
                        window.focus_window();

                        let now = std::time::Instant::now();
                        self.delta_time = (now - self.last_tick).as_secs_f64();

                        self.last_tick = now;
                    }
                }
            }

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *control_flow = ControlFlow::Exit,
                Event::WindowEvent {
                    window_id: _,
                    event: WindowEvent::Focused(focused),
                } => {
                    self.motions_on = focused;
                }
                _ => (),
            }
        });
    }

    fn handle_set_count(&mut self, pressed_keys: &Vec<Keycode>) {
        if pressed_keys.contains(&self.config.keybinds["count_0"]) {
            self.count = 0;
        }
        if pressed_keys.contains(&self.config.keybinds["count_1"]) {
            self.count = 2;
        }
        if pressed_keys.contains(&self.config.keybinds["count_2"]) {
            self.count = 3;
        }
        if pressed_keys.contains(&self.config.keybinds["count_3"]) {
            self.count = 6;
        }
        if pressed_keys.contains(&self.config.keybinds["count_4"]) {
            self.count = 10;
        }
    }

    fn handle_basic_motions(&mut self, pressed_keys: &Vec<Keycode>) {
        let speed = 150.0 * self.count as f64;
        let distance = speed * self.delta_time;
        if pressed_keys.contains(&self.config.keybinds["left"]) {
            self.mouse
                .move_to(
                    self.mouse_pos.x as i32 - distance as i32,
                    self.mouse_pos.y as i32,
                )
                .unwrap();
            self.mouse_pos.x -= distance;
        }
        if pressed_keys.contains(&self.config.keybinds["right"]) {
            self.mouse
                .move_to(
                    self.mouse_pos.x as i32 + distance as i32,
                    self.mouse_pos.y as i32,
                )
                .unwrap();
            self.mouse_pos.x += distance;
        }
        if pressed_keys.contains(&self.config.keybinds["down"]) {
            self.mouse
                .move_to(
                    self.mouse_pos.x as i32,
                    self.mouse_pos.y as i32 + distance as i32,
                )
                .unwrap();
            self.mouse_pos.y += distance;
        }
        if pressed_keys.contains(&self.config.keybinds["up"]) {
            self.mouse
                .move_to(
                    self.mouse_pos.x as i32,
                    self.mouse_pos.y as i32 - distance as i32,
                )
                .unwrap();
            self.mouse_pos.y -= distance;
        }

        // check if mouse_pos goes off screen
        let real_mouse_pos = self.mouse.get_position().unwrap();
        if self.mouse_pos.x as i32 > real_mouse_pos.x + 1 {
            self.mouse_pos.x = real_mouse_pos.x as f64;
        }
        if self.mouse_pos.y as i32 > real_mouse_pos.y + 1 {
            self.mouse_pos.y = real_mouse_pos.y as f64;
        }
        if (self.mouse_pos.x as i32) < real_mouse_pos.x - 1 {
            self.mouse_pos.x = real_mouse_pos.x as f64;
        }
        if (self.mouse_pos.y as i32) < real_mouse_pos.y - 1 {
            self.mouse_pos.y = real_mouse_pos.y as f64;
        }
    }

    fn handle_mouse_clicks(&mut self, pressed_keys: &Vec<Keycode>) {
        let button_pressed;
        let button;
        if pressed_keys.contains(&self.config.keybinds["right_click_modifier"]) {
            button_pressed = &mut self.right_pressed;
            button = Keys::RIGHT;
        } else {
            button_pressed = &mut self.left_pressed;
            button = Keys::LEFT;
        }

        let space_pressed = pressed_keys.contains(&self.config.keybinds["click"]);
        if space_pressed {
            if !*button_pressed {
                *button_pressed = true;
                self.mouse.click(&button).unwrap();
                std::thread::sleep(std::time::Duration::from_millis(100));
                self.mouse.click(&button).unwrap();
            }
        } else {
            *button_pressed = false;
        }
    }

    fn handle_scrolling(&mut self, pressed_keys: &Vec<Keycode>) {
        if pressed_keys.contains(&self.config.keybinds["scroll_up"]) {
            // Could be self.count but I think 1 is plenty
            self.mouse.wheel(1).unwrap();
        }
        if pressed_keys.contains(&self.config.keybinds["scroll_down"]) {
            // Could be self.count but I think -1 is plenty
            self.mouse.wheel(-1).unwrap();
        }
    }
}
