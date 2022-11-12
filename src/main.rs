use mouse_rs::{types::Point, Mouse};
use readkey::Keycode;


fn basic_motions(mouse: &Mouse, mouse_pos: &mut Point, delta_time: &f64) {
    let speed = 1000.0;
    let distance = (speed * delta_time) as i32;
    if Keycode::H.is_pressed() {
        mouse.move_to(mouse_pos.x - distance, mouse_pos.y).unwrap();
        mouse_pos.x -= distance;
    }
    if Keycode::L.is_pressed() {
        mouse.move_to(mouse_pos.x + distance, mouse_pos.y).unwrap();
        mouse_pos.x += distance;
    }
    if Keycode::J.is_pressed() {
        mouse.move_to(mouse_pos.x, mouse_pos.y + distance).unwrap();
        mouse_pos.y += distance;
    }
    if Keycode::K.is_pressed() {
        mouse.move_to(mouse_pos.x, mouse_pos.y - distance).unwrap();
        mouse_pos.y -= distance;
    }
}

fn mouse_clicks(mouse: &Mouse, left_pressed: &mut bool, right_pressed: &mut bool) {
    if Keycode::Space.is_pressed() {
        if !*left_pressed {
            mouse.press(&mouse_rs::types::keys::Keys::LEFT).unwrap();
            *left_pressed = true;
        }
    } else {
        if *left_pressed {
            mouse.release(&mouse_rs::types::keys::Keys::LEFT).unwrap();
            *left_pressed = false;
        }
    }
    // } else {
    //     mouse.release(&mouse_rs::types::keys::Keys::LEFT).unwrap();
    // }
}

fn main() {
    let mut motions_on = true;

    let mouse = Mouse::new();
    let mut delta_time = 0.0;
    let mut last_tick = std::time::Instant::now();
    let mut right_pressed = false;
    let mut left_pressed = false;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1));

        if Keycode::Escape.is_pressed() {
            motions_on = true;
        }

        if !motions_on {
            continue;
        }

        if Keycode::I.is_pressed() {
            motions_on = false;
        }

        let mut mouse_pos = mouse.get_position().unwrap();
        basic_motions(&mouse, &mut mouse_pos, &delta_time);
        mouse_clicks(&mouse, &mut left_pressed, &mut right_pressed);

        let now = std::time::Instant::now();
        delta_time = (now - last_tick).as_secs_f64();

        last_tick = now;
    }
}
