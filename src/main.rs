use crate::vec2::Vec2;
use mouse_rs::Mouse;
use readkey::Keycode;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub mod vec2;

fn set_count(count: &mut i32) {
    if Keycode::_0.is_pressed() {
        *count = 0;
    }
    if Keycode::_1.is_pressed() {
        *count = 1;
    }
    if Keycode::_2.is_pressed() {
        *count = 2;
    }
    if Keycode::_3.is_pressed() {
        *count = 3;
    }
    if Keycode::_4.is_pressed() {
        *count = 4;
    }
    if Keycode::_5.is_pressed() {
        *count = 5;
    }
    if Keycode::_6.is_pressed() {
        *count = 6;
    }
    if Keycode::_7.is_pressed() {
        *count = 7;
    }
    if Keycode::_8.is_pressed() {
        *count = 8;
    }
    if Keycode::_9.is_pressed() {
        *count = 9;
    }
    if Keycode::_9.is_pressed() {
        *count = 9;
    }
}

fn basic_motions(mouse: &Mouse, mouse_pos: &mut Vec2<f64>, delta_time: &f64, count: &mut i32) {
    let speed = 150.0 * *count as f64;
    let distance = speed * delta_time;
    if Keycode::H.is_pressed() {
        mouse
            .move_to(mouse_pos.x as i32 - distance as i32, mouse_pos.y as i32)
            .unwrap();
        mouse_pos.x -= distance;
    }
    if Keycode::L.is_pressed() {
        mouse
            .move_to(mouse_pos.x as i32 + distance as i32, mouse_pos.y as i32)
            .unwrap();
        mouse_pos.x += distance;
    }
    if Keycode::J.is_pressed() {
        mouse
            .move_to(mouse_pos.x as i32, mouse_pos.y as i32 + distance as i32)
            .unwrap();
        mouse_pos.y += distance;
    }
    if Keycode::K.is_pressed() {
        mouse
            .move_to(mouse_pos.x as i32, mouse_pos.y as i32 - distance as i32)
            .unwrap();
        mouse_pos.y -= distance;
    }
}

fn mouse_clicks(mouse: &Mouse, left_pressed: &mut bool, right_pressed: &mut bool) {
    use mouse_rs::types::keys::Keys;
    let key_pressed;
    let key;
    if Keycode::Shift.is_pressed() {
        key_pressed = right_pressed;
        key = Keys::RIGHT;
    } else {
        key_pressed = left_pressed;
        key = Keys::LEFT;
    }
    if Keycode::Space.is_pressed() {
        if !*key_pressed {
            mouse.press(&key).unwrap();
            *key_pressed = true;
        }
    } else {
        if *key_pressed {
            mouse.release(&key).unwrap();
            *key_pressed = false;
        }
    }
}

fn main() {
    let mut motions_on = true;

    let mouse = Mouse::new();
    let mut count = 6;
    let mut delta_time = 0.0;
    let mut last_tick = std::time::Instant::now();
    let mut right_pressed = false;
    let mut left_pressed = false;

    let mouse_pos = mouse.get_position().unwrap();
    let mut mouse_pos: Vec2<f64> = Vec2 {
        x: mouse_pos.x as f64,
        y: mouse_pos.y as f64,
    };

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
            motions_on = true;
        }

        if motions_on {
            // if Keycode::I.is_pressed() {
            //     // unfocus window?
            // }

            set_count(&mut count);
            basic_motions(&mouse, &mut mouse_pos, &delta_time, &mut count);
            mouse_clicks(&mouse, &mut left_pressed, &mut right_pressed);

            let now = std::time::Instant::now();
            delta_time = (now - last_tick).as_secs_f64();

            last_tick = now;
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
                motions_on = focused;
            }
            _ => (),
        }
    });
}
