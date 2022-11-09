use std::time::Duration;

use sdl2::{keyboard::Keycode, pixels::Color, render::Canvas, video::Window, Sdl, rect::Point};

pub(crate) enum Event {
    Clear,
    Draw([[u8;64];32]),
    Tick,
    Quit,
}

pub(crate) struct Display {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
}

impl Display {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Window", 64, 32)
            .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        Display {
            sdl_context,
            canvas,
        }
    }
    pub fn run(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();

        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                if let Some(user_event) = event.as_user_event_type() {
                    match user_event {
                        Event::Clear => {
                            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                            self.canvas.clear();
                        }
                        Event::Draw(dbuf) => {

                            let mut points = vec![];
                            for i in 0..32 {
                                for j in 0..64 {
                                    if dbuf[i][j] == 1 {
                                        points.push(Point::new(j as i32,i as i32));
                                    }
                                }
                            }
                            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                            self.canvas.draw_points(&points[..]).unwrap();
                        }
                        Event::Tick => (),
                        Event::Quit => break 'running,
                    }
                }
            }

            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        }
    }
}
