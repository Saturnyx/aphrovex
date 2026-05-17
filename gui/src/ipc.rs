use iceoryx2::{node::NodeBuilder, signal_handling_mode::SignalHandlingMode};
use roboscope_ipc::{Config, SimServices};

#[derive(Default)]
pub struct AppIpc {
    pub ipc:    Option<SimServices>,
    init_error: Option<String>,
}

impl AppIpc {
    pub fn ensure(&mut self) {
        if self.ipc.is_some() || self.init_error.is_some() {
            return;
        }

        if let Err(err) = self.try_init() {
            self.init_error = Some(err);
        }
    }

    pub fn init_error(&self) -> Option<&str> { self.init_error.as_deref() }

    pub fn ipc(&self) -> Option<&SimServices> { self.ipc.as_ref() }

    fn try_init(&mut self) -> Result<(), String> {
        let builder = NodeBuilder::new()
            .config(&Config::default())
            .signal_handling_mode(SignalHandlingMode::Disabled);

        let ipc = SimServices::custom(Some("viewer"), builder).map_err(|e| e.to_string())?;

        self.ipc = Some(ipc);

        Ok(())
    }
}
