extern crate sdl2;
use std::path::Path;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::image::{LoadTexture, INIT_PNG};
use sdl2::rect::{Rect, Point};




pub fn main() {
    let sdl_context = sdl2::init()
        .unwrap();

    let video_subsystem = sdl_context
        .video()
        .unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window
        .into_canvas()
        .build()
        .unwrap();
    let mut timer = sdl_context.timer().unwrap(); 
    let _image_context = sdl2::image::init(INIT_PNG).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(Path::new("assets/rogue.png")).unwrap();

    let character_size: (u32, u32) = (12, 15);
    let mut source_rect_0 = Rect::new(
                 character_size.0 as i32 * 3,
                 90,
                 character_size.0,
                 character_size.1);

    let mut dest_rect_0 = Rect::new(0, 0, 20, 20);
    dest_rect_0.center_on(Point::new(0,10));
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut new_x: i32 = 0;
    let mut new_y: i32 = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    new_y = dest_rect_0.y + character_size.1 as i32;
                },
                Event::KeyDown{ keycode: Some(Keycode::Up), .. } => {
                    new_y = dest_rect_0.y - character_size.1 as i32;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    new_x = dest_rect_0.x + character_size.0 as i32;
                },
                Event::KeyDown{ keycode: Some(Keycode::Left), .. } => {
                    new_x = dest_rect_0.x - character_size.0 as i32;
                },
                _ => {}
            }
        }
        
        let clamp = |value: i32, max_value: i32| {
            match value {
                value if value > max_value => 0,
                value if value < 0 => max_value,
                _ => value
            }
        };
        dest_rect_0.set_y(clamp(new_y, 600));
        dest_rect_0.set_x(clamp(new_x, 800));
        // The rest of the game loop goes here...
	    let index = {
            let ticks = timer.ticks() as i32;
            ticks / 100
        };
        let frame_index = {
            let frame_count = 5;
            let frame_offset = 2;
            index % frame_count + frame_offset
        };
        //println!("{}",character_size.0 as i32 * frame_index);
        source_rect_0.set_x(character_size.0 as i32 * frame_index);
        canvas.clear();
        canvas.copy_ex(
                &texture,
                Some(source_rect_0),
                Some(dest_rect_0), 0.0, None, false, false).unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::from_millis(100));
    }
}
