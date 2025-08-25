use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::{Mutex};//, Once};
//use once_cell::sync::OnceCell;

mod prime_calc;
mod primes_display;

use winit::dpi::{LogicalPosition, LogicalSize, PhysicalPosition};
use winit::event_loop::{ActiveEventLoop, EventLoop, ControlFlow};
use winit::application::ApplicationHandler;
use winit::raw_window_handle::HasDisplayHandle;
use winit::window::{Window, WindowId};
use winit::event::{WindowEvent,ElementState};
use wgpu;

use pixels::{Error, Pixels, SurfaceTexture};


//static WINDOWS: OnceCell<Mutex<HashMap<WindowId, Arc<Window>>>> = OnceCell::new();

const DEFOULT_W_H: (u32,u32) = (1920,1080);
const INITIAL_COLOR: [u8;4] = [0,0,0,255];
const CHANGE_COLOR: [u8;4] = [255,255,255,255];

struct BufferState {
    buffer_width: u32,
    buffer_hight: u32,
    current_buffer: Vec<u8>,
}

impl Default for BufferState {
    fn default() -> Self {
        let mut empty_buffer: Vec<u8> = Vec::new();
        for i in 0..DEFOULT_W_H.0*DEFOULT_W_H.1{
            empty_buffer.extend(INITIAL_COLOR);
        }
        Self {
            buffer_width: DEFOULT_W_H.0,
            buffer_hight: DEFOULT_W_H.1,
            current_buffer: empty_buffer, // [Value,number of]
        }
    }
}

struct WindowState {
    window: Arc<Window>,
    pixels: Pixels<'static>,
    buffer_state: BufferState,
    // We need to create a constructor for this
}

struct CursorState {
    physical_position: PhysicalPosition<f64>,
    logical_position: LogicalPosition<f64>,
    state: ElementState,
    window_presence: MouseInWindow,
}

impl CursorState {
}

impl Default for CursorState {
    fn default() -> Self {
        Self {
            physical_position: PhysicalPosition{ x: 0.0, y: 0.0 },
            logical_position: LogicalPosition{ x: 0.0, y: 0.0 },
            state: ElementState::Released,
            window_presence: MouseInWindow::Inside,
        }
    }
}

#[derive(PartialEq)]
enum MouseInWindow {
    Inside,
    Outside,
}

struct MainApp {//, W: wgpu::WindowHandle> {
    windows: HashMap<WindowId, WindowState>,
    main_window_id: Option<WindowId>,
    cursor: CursorState, 
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
        let mut pixels = Pixels::new(buffer_state.buffer_width, buffer_state.buffer_hight, surface).expect("create pixels");

        let pixel_frame = pixels.frame_mut();
        pixel_frame.copy_from_slice(&buffer_state.current_buffer);

        self.windows.insert(id, WindowState { window, pixels, buffer_state });
        id
    }

    fn set_color( buffer: &Vec<u8>, index: usize, color: (u8,u8,u8,u8) ) -> Vec<u8> {

        println!("The buffer size is: {}", buffer.len() );

        let mut result = buffer.clone();
        result[index] = color.0;
        result[index+1] = color.1;
        result[index+2] = color.2;
        result[index+3] = color.3;

        result
    }
}


impl Default for MainApp {
    fn default() -> Self {
        Self {
            windows: HashMap::new(),
            main_window_id: None,
            cursor: CursorState::default(),
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
                println!("Redraw Request!!!");
                self.windows.get(&window_id).as_ref().unwrap().pixels.render().expect("Rendering error");
            },
            WindowEvent::MouseInput { device_id, state, button } => {
                self.cursor.state = state;

                if (self.cursor.state == ElementState::Pressed) && (self.cursor.window_presence == MouseInWindow::Inside) {

                    //println!("Incoming variables: x - {} , y - {} , width - {}", self.cursor.physical_position.x, self.cursor.physical_position.y, self.windows.get(&self.main_window_id.unwrap()).unwrap().buffer_state.buffer_width);
                    
                    let index_based_on_mouse_position = ((self.cursor.physical_position.x+self.cursor.physical_position.y*(self.windows.get(&self.main_window_id.unwrap()).unwrap().buffer_state.buffer_width as f64)) as usize)*4;
                    println!( "Index based on calculation:{}", index_based_on_mouse_position );
                    let new_buffer = MainApp::set_color(&self.windows.get(&self.main_window_id.unwrap()).unwrap().buffer_state.current_buffer, index_based_on_mouse_position, (CHANGE_COLOR[0],CHANGE_COLOR[1],CHANGE_COLOR[2],CHANGE_COLOR[3])); 
                    
                    let pixel_frame = self.windows.get_mut(&self.main_window_id.unwrap()).unwrap().pixels.frame_mut();
                    pixel_frame.copy_from_slice(&new_buffer);

                     self.windows.get_mut(&window_id).as_mut().unwrap().buffer_state.current_buffer = new_buffer;

                    self.windows.get(&window_id).as_ref().unwrap().pixels.render().expect("Rendering error");
                }

                println!("Device {:?} has button {:?} change state to: {:?}", device_id, button, state);
            },
            WindowEvent::CursorEntered { device_id } => {
                self.cursor.window_presence = MouseInWindow::Inside;
                println!("Device {:?} has entered window {:?}.", device_id, window_id);
            },
            WindowEvent::CursorLeft { device_id } => {
                self.cursor.window_presence = MouseInWindow::Outside;
                println!("Device {:?} has left window {:?}.", device_id, window_id);
            },
            WindowEvent::CursorMoved{ device_id, position } => {
                self.cursor.physical_position = position;
                if ( self.cursor.state == ElementState::Pressed ){

                }

                println!("Device {:?} moved to postion: {:?} in screen id:{:?}", device_id, position, window_id);
            },
            window_event => {
                println!("Not used WindowEvent happend: {:?}", window_event);
            },
        }
    }
}


fn main() {

    //let mut prime_calculation = prime_calc::primes::PrimesCalcSettings::init(2, 10000, 300);
//
    //prime_calculation.start_calc();
    //prime_calculation.results.calculate_distances();

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut single_app = MainApp::default();

    event_loop.run_app(&mut single_app);
}
