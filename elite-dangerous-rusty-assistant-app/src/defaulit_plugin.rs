use ratatui::Frame;
use ratatui::layout::Rect;
use elite_dangerous_rusty_assistant_plugins::EliteDangerousPlugin;

pub struct DefaultPlugin {}

impl EliteDangerousPlugin for DefaultPlugin {
    fn name(&self) -> String {
        String::from("MockPlugin")
    }

    fn description(&self) -> String {
        String::from("Mock Plugin")
    }
}


