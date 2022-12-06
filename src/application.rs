use crate::vec2::Vec2;
use std::time::Instant;

use mouse_rs::{types::keys::Keys, Mouse};
use readkey::Keycode;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct Application {
    motions_on: bool,
    mouse: Mouse,
    count: i32,
    delta_time: f64,
    last_tick: Instant,
    right_pressed: bool,
    left_pressed: bool,
    i_pressed: bool,
    mouse_pos: Vec2<f64>,
}

impl Application {
    pub fn new() -> Self {
        let mouse = Mouse::new();
        let mouse_pos = mouse.get_position().unwrap();
        return Application {
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
        };
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(0.0, 0.0))
            .build(&event_loop)
            .unwrap();
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            std::thread::sleep(std::time::Duration::from_millis(1));

            if Keycode::Escape.is_pressed() {
                window.focus_window();
                self.motions_on = true;
            }

            if self.motions_on {
                if Keycode::I.is_pressed() {
                    // unfocus window
                    if !self.i_pressed {
                        self.mouse.click(&Keys::LEFT).unwrap();
                        self.motions_on = false;
                        self.i_pressed = true;
                    }
                } else {
                    self.i_pressed = false;
                    Self::handle_set_count(&mut self);
                    self.handle_basic_motions();
                    self.handle_mouse_clicks();
                    window.focus_window();

                    let now = std::time::Instant::now();
                    self.delta_time = (now - self.last_tick).as_secs_f64();

                    self.last_tick = now;
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

    fn handle_set_count(&mut self) {
        if Keycode::_0.is_pressed() {
            self.count = 0;
        }
        if Keycode::_1.is_pressed() {
            self.count = 1;
        }
        if Keycode::_2.is_pressed() {
            self.count = 3;
        }
        if Keycode::_3.is_pressed() {
            self.count = 6;
        }
        if Keycode::_4.is_pressed() {
            self.count = 10;
        }
    }

    fn handle_basic_motions(&mut self) {
        let speed = 150.0 * self.count as f64;
        let distance = speed * self.delta_time;
        if Keycode::H.is_pressed() {
            self.mouse
                .move_to(
                    self.mouse_pos.x as i32 - distance as i32,
                    self.mouse_pos.y as i32,
                )
                .unwrap();
            self.mouse_pos.x -= distance;
        }
        if Keycode::L.is_pressed() {
            self.mouse
                .move_to(
                    self.mouse_pos.x as i32 + distance as i32,
                    self.mouse_pos.y as i32,
                )
                .unwrap();
            self.mouse_pos.x += distance;
        }
        if Keycode::J.is_pressed() {
            self.mouse
                .move_to(
                    self.mouse_pos.x as i32,
                    self.mouse_pos.y as i32 + distance as i32,
                )
                .unwrap();
            self.mouse_pos.y += distance;
        }
        if Keycode::K.is_pressed() {
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
    fn handle_mouse_clicks(&mut self) {
        let button_pressed;
        let button;
        if Keycode::Shift.is_pressed() {
            button_pressed = &mut self.right_pressed;
            button = Keys::RIGHT;
        } else {
            button_pressed = &mut self.left_pressed;
            button = Keys::LEFT;
        }

        let space_pressed = Keycode::Space.is_pressed();
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
}