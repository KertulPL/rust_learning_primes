use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::{Mutex};//, Once};
//use once_cell::sync::OnceCell;

mod prime_calc;
mod primes_display;

use winit::dpi::LogicalSize;
use winit::event_loop::{ActiveEventLoop, EventLoop, ControlFlow};
use winit::application::ApplicationHandler;
use winit::raw_window_handle::HasDisplayHandle;
use winit::window::{Window, WindowId};
use winit::event::WindowEvent;
use wgpu;

use pixels::{Error, Pixels, SurfaceTexture};


//static WINDOWS: OnceCell<Mutex<HashMap<WindowId, Arc<Window>>>> = OnceCell::new();

const  DEFOULT_W_H: (u32,u32) = (1920,1080);

struct BufferState {
    buffer_width: u32,
    buffer_hight: u32,
}

impl Default for BufferState {
    fn default() -> Self {
        Self {
            buffer_width: DEFOULT_W_H.0,
            buffer_hight: DEFOULT_W_H.1,
        }
    }
}

struct WindowState {
    window: Arc<Window>,
    pixels: Pixels<'static>,
    buffer_state: BufferState,
    // We need to create a constructor for this
}

struct MainApp {//, W: wgpu::WindowHandle> {
    windows: HashMap<WindowId, WindowState>,
    main_window_id: Option<WindowId>, // Moze poprostu adres [0] w vectorze jest już przypisany do głównego ekranu ?
    //surface: Option<SurfaceTexture<W>>,
}

impl MainApp{
    fn create_window(&mut self, el: &ActiveEventLoop, title: &str, buffer_state: BufferState) -> WindowId {

        let window = Arc::new(
            el.create_window(
                Window::default_attributes()
                    .with_title(title)
                    .with_inner_size(LogicalSize::new(buffer_state.buffer_width, buffer_state.buffer_hight)),
            )
            .expect("create window"),
        );
        let id = window.id();
        let size = window.inner_size();

        let surface = SurfaceTexture::new(size.width, size.height, window.clone());
        let pixels = Pixels::new(buffer_state.buffer_width, buffer_state.buffer_hight, surface).expect("create pixels");

        self.windows.insert(id, WindowState { window, pixels, buffer_state });
        id
    }
}


impl Default for MainApp {
    fn default() -> Self {
        Self {
            windows: HashMap::new(),
            main_window_id: None,
        }
    }
}

impl ApplicationHandler for MainApp {
    fn resumed(
        &mut self,
        event_loop: &ActiveEventLoop
    ){

        if self.windows.len() != 0 {
            // logic for resuming all windows
        }
        else{
            
            self.main_window_id = Some( self.create_window(event_loop, "Main Window", BufferState::default()) );

            match self.windows.get(self.main_window_id.as_ref().unwrap()).unwrap().pixels.render() { // We should remove the unwrap soon ... :P
                Ok( result ) => {
                    println!("Window render result: {:?}", result);
                },
                Err( error ) => {
                    println!("Window render Error: {:?}", error);
                }
            }

        }
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ){
        match event {
            WindowEvent::CloseRequested => {
                if Some(window_id) == self.main_window_id {
                    println!("Main screen close button pressed. We stop entire app.");
                    event_loop.exit();
                }
                else {
                    println!("The close button was pressed for window: {:?}; stopping the window", window_id);
                    match self.windows.remove(&window_id){
                        Some( window_state ) => {
                            drop(window_state); 
                        },
                        None => {},
                    }
                }
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
                self.windows.get(&window_id).as_ref().unwrap().pixels.render().expect("Rendering error");
            },
            WindowEvent::MouseInput { device_id, state, button } => {
                println!("Device {:?} has button {:?} change state to: {:?}", device_id, button, state);
            },
            WindowEvent::CursorEntered { device_id } => {
                println!("Device {:?} has entered window {:?}.", device_id, window_id);
            },
            WindowEvent::CursorLeft { device_id } => {
                println!("Device {:?} has left window {:?}.", device_id, window_id);
            },
            WindowEvent::CursorMoved{ device_id, position } => {
                println!("Device {:?} moved to postion: {:?} in screen id:{:?}", device_id, position, window_id);
            },
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

    let mut single_app = MainApp::default();

    event_loop.run_app(&mut single_app);
}
