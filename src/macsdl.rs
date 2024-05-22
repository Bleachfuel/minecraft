use sdl2::{event::Event, keyboard::Keycode, video::Window, EventPump, Sdl};


pub struct Macsdl {
    pub sdl: Sdl,
    pub window: Window,
    pub event_pump: EventPump
}

impl Macsdl {
    pub fn init (title: &str, width: usize, height: usize) -> Result<Self, &'static str> {
        let sdl = sdl2::init().expect("Could not initialize sdl");
        let video_subsystem = sdl.video().expect("Could not connect with video subsystem");
        let window = video_subsystem
            .window(title, width.try_into().unwrap(), height.try_into().unwrap())
            .resizable()
            .build()
            .expect("Could not create sdl window");
        let event_pump = sdl.event_pump().expect("Could not create sdl event pump");
        Ok( Macsdl {
            sdl,
            window,
            event_pump
        }
        )
    }

    pub fn handle_events(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => return false,
                _ => {}
            }
        }
        true
    }
}