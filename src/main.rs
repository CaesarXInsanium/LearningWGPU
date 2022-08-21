mod render_pipe;
mod app;
mod vertex;
use app::AppState;
use render_pipe::RenderPipeBuilder;
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
    let mut render_pipe_builder = pollster::block_on(RenderPipeBuilder::new(&window));

    render_pipe_builder.add_mesh(vertex::VERTICES, vertex::INDICES);

    render_pipe_builder.add_texture(happy_tree_bytes, Some("Happy Tree"));

    let render_pipe = render_pipe_builder.build();

    let mut app = AppState::new(render_pipe);
    event_loop.run(move |event, _, control_flow| {
        app.handle_event_cycle(&window, event, control_flow);
    });
}
