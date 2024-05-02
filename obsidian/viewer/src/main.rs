// obsidian/viewer/src/main.rs
use anyhow::Result;
use log::info;
use obsidian::app::{run_application, AppConfig, Application, Run};

pub struct Viewer;

impl Run for Viewer {
    fn initialize(&mut self, _application: &mut Application) -> Result<()> {
        info!("Viewer initialized");
        Ok(())
    }

    fn update(&mut self, _application: &mut Application) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    let viewer = Viewer {};
    run_application(
        viewer,
        AppConfig {
            title: "Obsidian Viewer".to_string(),
            logfile_name: "viewer.log".to_string(),
            ..Default::default()
        },
    )
}
