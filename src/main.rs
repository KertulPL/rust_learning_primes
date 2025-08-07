mod prime_calc;
mod primes_display;

use winit::event_loop::{ActiveEventLoop, EventLoop, ControlFlow};
use winit::application::ApplicationHandler;
use winit::window::{Window, WindowId};
use winit::event::WindowEvent;

#[derive(Default)]
struct MainApp {
    window: Option<Window>,

}

impl ApplicationHandler for MainApp {
    fn resumed(
        &mut self,
        event_loop: &ActiveEventLoop
    ){
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ){
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}


fn main() {

    let mut prime_calculation = prime_calc::primes::PrimesCalcSettings::init(2, 10000, 300);

    prime_calculation.start_calc();
    prime_calculation.results.calculate_distances();

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut single_app = MainApp::default(); //MainApp{ window: None };
    //event_loop.create_window(_);

    event_loop.run_app(&mut single_app);
}
