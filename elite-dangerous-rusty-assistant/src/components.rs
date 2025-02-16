use std::sync::Arc;
use elite_dangerous_journal_model::events::EliteDangerousEvent;
use crate::components::pirate_massacre::component::PirateMassacreComponent;
use crate::traits::EDRAComponent;

pub(crate) mod pirate_massacre;

pub enum EDRAFunction {
    PirateMassacre(PirateMassacreComponent)
}

impl EDRAComponent for EDRAFunction {
    fn name(&self) -> String {
        match self {
            EDRAFunction::PirateMassacre(pirate_massacre) => pirate_massacre.name()        }
    }

    fn description(&self) -> String {
        match self {
            EDRAFunction::PirateMassacre(pirate_massacre) => pirate_massacre.description()
        }
    }

    async fn process_event(&self, event: Arc<EliteDangerousEvent>) -> Result<(), String> {
        match self {
            EDRAFunction::PirateMassacre(pirate_massacre) => pirate_massacre.process_event(event).await
        }
    }

    async fn render(&self) {
        match self {
            EDRAFunction::PirateMassacre(pirate_massacre) => pirate_massacre.render().await
        }
    }
}