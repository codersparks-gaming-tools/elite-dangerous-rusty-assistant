use std::path::PathBuf;
use std::sync::Arc;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;
use tokio::task::JoinSet;
use elite_dangerous_journal_model::events::EliteDangerousEvent;
use crate::components::EDRAFunction;
use crate::components::EDRAFunction::PirateMassacre;
use crate::components::pirate_massacre::component::PirateMassacreComponent;
use crate::traits::EDRAComponent;

pub mod components;
pub mod traits;

pub struct EliteDangerousRustyAssistant {
    pub db: Arc<Surreal<Db>>,
    components: Vec<Arc<EDRAFunction>>,
    component_loaded_index: usize,
}

impl EliteDangerousRustyAssistant {
    pub async fn new(db_dir: PathBuf) -> Self {

        let db : Surreal<Db> = Surreal::new::<RocksDb>(db_dir.clone()).await.expect("Failed to create surreal db handle");

        let db_ref = Arc::new(db);

        let pirate_component = PirateMassacreComponent::new(db_ref.clone());

        let components = vec![Arc::new(PirateMassacre(pirate_component))];

        Self {
            db: db_ref,
            components,
            component_loaded_index: 0,
        }
    }

    pub fn change_loaded_component(&mut self, new_index: usize) -> Result<(), String> {
        if new_index < self.components.len() {
            self.component_loaded_index = new_index;
            Ok(())
        } else {
            Err(format!("Invalid component index: {}", new_index))
        }
    }

    pub fn get_loaded_component(&self) -> Arc<EDRAFunction> {
        self.components[self.component_loaded_index].clone()
    }
}

impl EDRAComponent for EliteDangerousRustyAssistant {
    fn name(&self) -> String {
        "Elite Dangerous Rusty Assistant".to_string()
    }
    
    fn description(&self) -> String {
        "An assistant for elite dangerous written in rust".to_string()
    }

    async fn process_event(&self, event: Arc<EliteDangerousEvent>) -> Result<(), String>{
        let mut task_set = JoinSet::new();
        self.components.iter().for_each(|c| {
            let e = event.clone();
            let component = c.clone();
            task_set.spawn(async move {
                match component.process_event(e).await {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error processing event: {}", e);
                    }
                }
            });
        });
        
        task_set.join_all().await;
        
        Ok(())
    }

    async fn render(&self) {
        self.get_loaded_component().render().await;
    }
}