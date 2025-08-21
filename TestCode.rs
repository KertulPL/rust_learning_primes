use std::collections::HashMap;
use std::sync::Arc;

use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes, WindowId},
};

const BUF_W: u32 = 320;
const BUF_H: u32 = 240;

// GLOBAL: Window store (id -> Arc<Window>)
static WINDOWS: OnceCell<Mutex<HashMap<WindowId, Arc<Window>>>> = OnceCell::new();

struct App {
    // Persistent renderers per window (no borrows → 'static)
    pixels_by_id: HashMap<WindowId, Pixels<'static>>,
}

impl Default for App {
    fn default() -> Self {
        WINDOWS.set(Mutex::new(HashMap::new())).ok(); // ignore if already set on resume
        Self { pixels_by_id: HashMap::new() }
    }
}

impl App {
    fn create_window(&mut self, el: &ActiveEventLoop, title: &str) -> WindowId {
        // 1) Create a Window
        let window = el
            .create_window(
                WindowAttributes::default()
                    .with_title(title)
                    .with_inner_size(LogicalSize::new((BUF_W * 2) as f64, (BUF_H * 2) as f64)),
            )
            .expect("create window");
        let id = window.id();

        // 2) Store it globally as Arc<Window>
        let win_arc = Arc::new(window);
        WINDOWS.get().unwrap().lock().insert(id, win_arc.clone());

        // 3) Build a SurfaceTexture using OWNED handle (Arc<Window>)
        let size = win_arc.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, win_arc);

        // 4) Create persistent Pixels<'static> (no lifetime pain)
        let px = Pixels::new(BUF_W, BUF_H, surface).expect("create pixels");
        self.pixels_by_id.insert(id, px);

        // 5) Kick first redraw
        if let Some(win) = WINDOWS.get().unwrap().lock().get(&id) {
            win.request_redraw();
        }
        id
    }

    fn destroy_window(&mut self, id: WindowId) {
        // Drop Pixels first
        self.pixels_by_id.remove(&id);
        // Remove Window from global map
        WINDOWS.get().unwrap().lock().remove(&id);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.pixels_by_id.is_empty() {
            self.create_window(el, "Window 1");
            self.create_window(el, "Window 2");
        }
    }

    fn window_event(
        &mut self,
        el: &ActiveEventLoop,
        id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.destroy_window(id);
                if self.pixels_by_id.is_empty() {
                    el.exit();
                }
            }

            WindowEvent::Resized(sz) => {
                if let Some(p) = self.pixels_by_id.get_mut(&id) {
                    p.resize_surface(sz.width, sz.height);
                }
            }

            WindowEvent::KeyboardInput { event, .. } => {
                // Press 'N' to spawn a new window at runtime
                if event.logical_key == Key::Character("n".into()) {
                    let n = self.pixels_by_id.len() + 1;
                    self.create_window(el, &format!("Window {}", n));
                }
                // ESC: close this window
                if event.logical_key == Key::Named(NamedKey::Escape) {
                    self.destroy_window(id);
                    if self.pixels_by_id.is_empty() {
                        el.exit();
                    }
                }
            }

            WindowEvent::RedrawRequested => {
                if let Some(pixels) = self.pixels_by_id.get_mut(&id) {
                    // Fill frame (black)
                    let frame = pixels.get_frame();
                    for px in frame.chunks_exact_mut(4) {
                        px.copy_from_slice(&[0, 0, 0, 255]);
                    }
                    pixels.render().expect("render");
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _el: &ActiveEventLoop) {
        // Ask all windows to redraw continuously (or schedule selectively if you prefer)
        for win in WINDOWS.get().unwrap().lock().values() {
            win.request_redraw();
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let event_loop = EventLoop::new()?;
    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}
