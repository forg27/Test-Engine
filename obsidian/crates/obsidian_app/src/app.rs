// crates/obsidian_app/src/app.rs
use anyhow::Result;
use log::{error, info};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct AppConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub logfile_name: String,
}

pub fn create_window(config: &AppConfig) -> Result<(EventLoop<()>, Window)> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title(config.title.to_string())
        .with_inner_size(PhysicalSize::new(config.width, config.height))
        .build(&event_loop)?;

    Ok((event_loop, window))
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "Obsidian Application".to_string(),
            logfile_name: "obsidian.log".to_string(),
        }
    }
}

pub struct Application;

pub trait Run {
    fn initialize(&mut self, _application: &mut Application) -> Result<()> {
        Ok(())
    }

    fn update(&mut self, _application: &mut Application) -> Result<()> {
        Ok(())
    }
}

pub fn run_application(mut runner: impl Run + 'static, configuration: AppConfig) -> Result<()> {
    let (event_loop, _window) = create_window(&configuration)?;

    let mut application = Application {};

    log::info!("Running Application");
    runner.initialize(&mut application)?;

    event_loop.run(move |event, _, control_flow| {
        let mut cycle_result = || -> Result<()> {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::MainEventsCleared => {
                    runner.update(&mut application)?;
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::LoopDestroyed => {
                    info!("Exited application");
                }
                _ => {}
            }
            Ok(())
        };
        if let Err(error) = cycle_result() {
            error!("Application Error: {}", error);
        }
    });
}
