use std::time::{Instant, Duration};
use color_eyre::eyre::Result;
use tracing::{Level, error, info, span};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod types;
mod renderer;
mod camera;
use renderer::GfxState;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let evt_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&evt_loop)?;

    let mut gfx_state = pollster::block_on(GfxState::new(&window))?;

    let mut last_frame = Instant::now();
    evt_loop.run(move |event, _, ctl_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !gfx_state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *ctl_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physize) => {
                            gfx_state.resize(*physize);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            gfx_state.resize(**new_inner_size);
                        }
                        _ => (),
                    }
                }
            }

            Event::MainEventsCleared => {
            //     im_pipe.platform.prepare_frame(imgui.io_mut(), &window).expect("couldn't prepare imgui frame");
                window.request_redraw(); // always request a new frame
            }

            Event::RedrawRequested(_) => {
                let now = Instant::now();
                let delta_s = now - last_frame;
            //     imgui.io_mut().update_delta_time(delta_s);
                last_frame = now;

                gfx_state.update();
                match gfx_state.render() {
                    Ok(_) => {},
                    // lost the surface? try to reconfig
                    Err(wgpu::SurfaceError::Lost) => gfx_state.resize(gfx_state.size),
                    // GPU OOM? shut down and be angry
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        error!("wgpu device OOM");
                        *ctl_flow = ControlFlow::Exit;
                    },
                    Err(e) => error!(error=?e, "wgpu/render error")
                }
            }

            _ => {},
        }
    });

    Ok(())
}
