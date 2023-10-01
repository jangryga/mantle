use tao::event_loop::{EventLoop, ControlFlow};
use tao::window::WindowBuilder;
use tao::event::{Event, WindowEvent};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit;
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                println!("redraw requested!");
            },
            _ => ()
        }

    })
}