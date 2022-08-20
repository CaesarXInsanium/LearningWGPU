mod app;
mod context;
mod vertex;
use context::Context;
use app::AppState;
use winit::{event_loop::EventLoop, window::WindowBuilder, dpi::LogicalSize};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

/// this is the main fuction
fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("WGPU Learning")
        .with_resizable(false)
        .with_inner_size(LogicalSize {
            width: WIDTH,
            height: HEIGHT,
        })
        .build(&event_loop)
        .unwrap();
    // Textures
    let happy_tree_bytes = include_bytes!("happy-tree.png");
    
    // WGPU
    let mut context = pollster::block_on(Context::new(&window));
    context.add_mesh(vertex::VERTICES, vertex::INDICES);

    context.add_texture(happy_tree_bytes);

    let mut app = AppState::new(context);
    event_loop.run(move |event, _, control_flow| {
        app.handle_event_cycle(&window, event, control_flow);
    });
}
