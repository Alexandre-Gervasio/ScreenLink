//! Plugin system module
//! Extensibility through plugins

use crate::InputEvent;

pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn on_event(&mut self, event: InputEvent) -> InputEvent;
    fn on_load(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn on_unload(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Registering plugin: {}", plugin.name());
        plugin.as_ref().name(); // Keep reference
        self.plugins.push(plugin);
        Ok(())
    }

    pub fn process_event(&mut self, mut event: InputEvent) -> InputEvent {
        for plugin in &mut self.plugins {
            event = plugin.on_event(event);
        }
        event
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}