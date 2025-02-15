use elite_dangerous_journal_model::events::exploration::scan::event::PlanetMoon;

pub struct ExobiologyTarget {
    pub body_data: PlanetMoon,
    pub biology_signals: Vec<String>,
    pub all_bodies_found: bool,
    
}