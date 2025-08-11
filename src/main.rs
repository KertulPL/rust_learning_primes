mod prime_calc;
mod primes_display;

use winit::dpi::LogicalSize;
use winit::event_loop::{ActiveEventLoop, EventLoop, ControlFlow};
use winit::application::ApplicationHandler;
use winit::raw_window_handle::HasDisplayHandle;
use winit::window::{Window, WindowId};
use winit::event::WindowEvent;

use pixels::{Error, Pixels, SurfaceTexture};

const BUF_W: u32 = 1920;
const BUF_H: u32 = 1080;

struct MainApp<'app> {
    window: Option<Window>,
    pixels: Option<Pixels<'app>>,
    surface: Option<SurfaceTexture>,
}

impl <'app> Default for MainApp<'app> {
    fn default() -> Self {
        Self {
            window: None,
            pixels: None,
        }
    }
}

impl <'app> ApplicationHandler for MainApp<'app> {
    fn resumed(
        &mut self,
        event_loop: &ActiveEventLoop
    ){

        self.window = Some(event_loop.create_window(Window::default_attributes().with_title("Kertul Test App").with_inner_size(LogicalSize::new(BUF_W,BUF_H))).expect("Resumed: window creation failed."));

        let size = self.window.as_ref().expect("Resume: No window").inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, self.window.as_ref().expect("Resume: No window"));
        let px = Box::new(Some(Pixels::new(BUF_W, BUF_H, surface).expect("create pixels")));
        self.pixels = px;//Pixels::new(BUF_W, BUF_H, surface).expect("create pixels"));
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
            WindowEvent::CursorMoved{ device_id, position } => {
                println!("Device {:?} moved to postion: {:?}", device_id, position);
            },
            _ => (),
        }
    }
}


fn main() {

    //let mut prime_calculation = prime_calc::primes::PrimesCalcSettings::init(2, 10000, 300);

    //prime_calculation.start_calc();
    //prime_calculation.results.calculate_distances();

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut single_app = MainApp::default();

    event_loop.run_app(&mut single_app);
}
