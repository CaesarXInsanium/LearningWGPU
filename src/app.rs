use crate::render_pipe::RenderPipe;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

pub struct AppState {
    pub render_pipe: RenderPipe,
}
impl AppState {
    pub fn new(context: RenderPipe) -> Self {
        Self { render_pipe: context }
    }
    pub fn handle_event_cycle(
        &mut self,
        window: &Window,
        event: Event<()>,
        control_flow: &mut ControlFlow,
    ) {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    self.render_pipe.resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    self.render_pipe.resize(**new_inner_size);
                }
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                match self.render_pipe.render() {
                    Ok(_) => {}
                    // surface is lost
                    Err(wgpu::SurfaceError::Lost) => self.render_pipe.resize(self.render_pipe.size),
                    // No more memory
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // unknown Error
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    }
}
