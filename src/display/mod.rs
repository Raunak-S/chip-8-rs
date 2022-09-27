use sdl2::{render::Canvas, video::Window, pixels::Color, event::Event, keyboard::Keycode, Sdl};

pub(crate) struct Display {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>
}


impl Display {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Window", 64, 32)
            .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas()
            .build()
            .unwrap();

        Display {
            sdl_context,
            canvas
        }
    }
    pub fn demo(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.clear();
        self.canvas.present();

        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..}
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
        }
    }
    pub fn run(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.clear();
        self.canvas.present();

        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..}
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
        }
    }
}