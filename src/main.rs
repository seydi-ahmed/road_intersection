extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

// ******************************* AJOUTÉ ********************************************

pub struct Road {
    pub start: (i32, i32),
    pub end: (i32, i32),
}

impl Road {
    pub fn new(start: (i32, i32), end: (i32, i32)) -> Road {
        Road { start, end }
    }
}

pub fn draw_road(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, road: &Road) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_line(road.start, road.end).unwrap();
}

// ******************************* AJOUTÉ ********************************************


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 800)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // ***************************** Coordonnées ***********************************

    // let road_north_south = Road::new((400, 0), (400, 800));
    // let road_east_west = Road::new((0, 400), (800, 400));
    let lines = vec![
        Road::new((300, 0), (300, 300)), //done
        Road::new((400, 0), (400, 300)), //done
        Road::new((500, 0), (500, 300)), //done
        Road::new((500, 300), (800, 300)), //done
        Road::new((500, 400), (800, 400)), //done
        Road::new((500, 500), (800, 500)), //done
        Road::new((500, 500), (500, 800)), //done
        Road::new((400, 500), (400, 800)), //done
        Road::new((300, 500), (300, 800)), //done
        Road::new((300, 500), (0, 500)), //done
        Road::new((0, 400), (300, 400)), //done
        Road::new((300, 300), (0, 300)),
    ];


    // ***************************** Coordonnées ***********************************

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        
        // ************************** ROAD INERSECTION *******************************
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // draw_road(&mut canvas, &road_north_south);
        // draw_road(&mut canvas, &road_east_west);
        for line in &lines {
            draw_road(&mut canvas, &line);
        }
        

        // ************************** ROAD INERSECTION *******************************


        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


// les positions des points et des lignes:
// 1) ligne 1:
// a) start: 0, 200
// b) end: 200, 200

// 1) ligne 2:
// a) start: 0, 400
// b) end: 200, 400

// 1) ligne 3:
// a) start: 0, 600
// b) end: 200, 600

// 1) ligne 4:
// a) start: 600, 200
// b) end: 800, 200

// 1) ligne 5:
// a) start: 600, 400
// b) end: 800, 400

// 1) ligne 6:
// a) start: 600, 600
// b) end: 800, 600

// 1) ligne 7:
// a) start: 600, 600
// b) end: 600, 800

// 1) ligne 8:
// a) start: 400, 600
// b) end: 400, 800

// 1) ligne 9:
// a) start: 200, 600
// b) end: 200, 800

// 1) ligne 10:
// a) start: 200, 600
// b) end: 0, 600

// 1) ligne 11:
// a) start: 200, 400
// b) end: 0, 400

// 1) ligne 12:
// a) start: 200, 200
// b) end: 0, 200
