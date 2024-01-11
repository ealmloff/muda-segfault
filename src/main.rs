use muda::*;
use tao::{
    event::{Event, StartCause},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::default().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                let menu = Menu::new();

                // It can panic when only one menu item is added, but it is much more common when many items are added at once.
                menu.append_items(&[
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::separator(),
                ])
                .unwrap();

                menu.init_for_nsapp();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}
