use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowBuilder},
};

const HEIGHT: u32 = 600;
const WIDTH: u32 = 800;

struct HelloTriangleApplication;

impl HelloTriangleApplication {
  fn init_vulkan(&mut self) {}

  fn init_window(&mut self, event_loop: &EventLoop<()>) -> Window {
    WindowBuilder::new()
      .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
      .with_resizable(false)
      .with_title("Vulkan")
      .build(event_loop)
      .expect("failed to create window")
  }

  fn main_loop(&self, event_loop: EventLoop<()>) {
    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Wait;

      match event {
          Event::WindowEvent {
              event: WindowEvent::CloseRequested,
              ..
          } => *control_flow = ControlFlow::Exit,
          _ => (),
      }
    });
  }

  fn run(&mut self) {
    let event_loop = EventLoop::new();

    // Save window reference to keep it alive
    let _window = self.init_window(&event_loop);
    self.init_vulkan();
    self.main_loop(event_loop);
  }
}

fn main() {
  let mut app = HelloTriangleApplication;
  app.run();
}
