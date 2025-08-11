use once_cell::sync::OnceCell;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

static WINDOW: OnceCell<Window> = OnceCell::new();

const BUF_W: u32 = 320;
const BUF_H: u32 = 240;

struct App {
    window_id: Option<WindowId>,
    pixels: Option<Pixels<'static>>,
    // Persistent canvas so clicks remain drawn:
    canvas: Vec<u8>, // RGBA, size = BUF_W * BUF_H * 4
    // Last known cursor position in physical pixels:
    last_cursor: (f64, f64),
}

impl Default for App {
    fn default() -> Self {
        Self {
            window_id: None,
            pixels: None,
            canvas: vec![0; (BUF_W * BUF_H * 4) as usize], // black
            last_cursor: (0.0, 0.0),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window and stash globally
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("Click to light a pixel (white)")
                    .with_inner_size(LogicalSize::new(BUF_W as f64 * 2.0, BUF_H as f64 * 2.0)),
            )
            .expect("create window");
        self.window_id = Some(window.id());
        let _ = WINDOW.set(window);

        // Hook up pixels to window
        let win = WINDOW.get().expect("window set");
        let size = win.inner_size(); // physical size
        let surface = SurfaceTexture::new(size.width, size.height, win);
        let px = Pixels::new(BUF_W, BUF_H, surface).expect("create pixels");
        self.pixels = Some(px);

        // First redraw
        win.request_redraw();
    }

    fn window_event(
        &mut self,
        el: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if Some(window_id) != self.window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => el.exit(),

            WindowEvent::CursorMoved { position, .. } => {
                self.last_cursor = (position.x, position.y); // physical coords
            }

            WindowEvent::MouseInput { state, button, .. } => {
                if state == ElementState::Pressed {
                    if matches!(button, MouseButton::Left | MouseButton::Right | MouseButton::Middle)
                    {
                        // Map physical cursor coords -> buffer coords
                        if let Some(win) = WINDOW.get() {
                            let size = win.inner_size(); // physical size of surface
                            let (px, py) = self.last_cursor;
                            if size.width > 0 && size.height > 0 {
                                let x = ((px / size.width as f64) * BUF_W as f64) as i32;
                                let y = ((py / size.height as f64) * BUF_H as f64) as i32;
                                self.set_pixel(x, y, [255, 255, 255, 255]); // white
                                win.request_redraw();
                            }
                        }
                    }
                }
            }

            WindowEvent::Resized(size) => {
                // Keep logical buffer the same; just resize the surface
                if let Some(p) = self.pixels.as_mut() {
                    p.resize_surface(size.width, size.height);
                }
            }

            WindowEvent::RedrawRequested => {
                if let Some(pixels) = self.pixels.as_mut() {
                    // Copy persistent canvas into this frame and present
                    let frame = pixels.get_frame();
                    frame.copy_from_slice(&self.canvas);
                    pixels.render().expect("render");
                }
            }

            _ => {}
        }
    }
}

impl App {
    fn set_pixel(&mut self, x: i32, y: i32, rgba: [u8; 4]) {
        if x < 0 || y < 0 {
            return;
        }
        let (x, y) = (x as u32, y as u32);
        if x >= BUF_W || y >= BUF_H {
            return;
        }
        let idx = ((y * BUF_W + x) * 4) as usize;
        self.canvas[idx..idx + 4].copy_from_slice(&rgba);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let event_loop = EventLoop::new()?;
    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}
