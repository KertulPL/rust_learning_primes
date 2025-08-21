use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};
use pixels::{Pixels, SurfaceTexture};

const BUF_W: u32 = 1920;
const BUF_H: u32 = 1080;

struct MainApp<'a> {
    window: Option<Window>,
    pixels: Option<Pixels<'a>>,
}

impl<'a> Default for MainApp<'a> {
    fn default() -> Self {
        Self { window: None, pixels: None }
    }
}

impl<'a> ApplicationHandler for MainApp<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // 1) Tworzymy okno raz
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Kertul Test App")
                    .with_inner_size(LogicalSize::new(BUF_W, BUF_H)),
            )
            .expect("window creation failed");
        let size = window.inner_size();

        // 2) Tworzymy SurfaceTexture raz i OD RAZU przekazujemy do Pixels::new
        let surface = SurfaceTexture::new(size.width, size.height, &window);
        let pixels = Pixels::new(BUF_W, BUF_H, surface).expect("create pixels");

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            // Reagujemy na resize — NIE tworzymy SurfaceTexture na nowo,
            // tylko prosimy Pixels o zmianę rozmiaru powierzchni.
            WindowEvent::Resized(new_size) => {
                if let Some(p) = self.pixels.as_mut() {
                    // Uwaga: to są fizyczne piksele, użyj new_size.width/height.
                    let _ = p.resize_surface(new_size.width, new_size.height);
                }
                if let Some(w) = self.window.as_ref() {
                    w.request_redraw();
                }
            }

            WindowEvent::RedrawRequested => {
                if let Some(p) = self.pixels.as_mut() {
                    // Rysowanie do bufora (tu: czyszczenie na czarno)
                    let frame = p.frame_mut();
                    frame.fill(0);

                    // Prezentacja
                    if let Err(e) = p.render() {
                        eprintln!("pixels.render error: {e}");
                    }
                }
            }

            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = MainApp::default();
    event_loop.run_app(&mut app).unwrap();
}
