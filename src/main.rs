use std::{thread, time::Duration};
use rustcraft::macsdl::Macsdl;




fn main() {
    let mut ctx = Macsdl::init("minecraft clone", 500, 500).expect("could not create minecraft");


    'running: loop {
        if !ctx.handle_events() {
            break 'running;
        }

        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
