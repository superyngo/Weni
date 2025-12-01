use serde::Serialize;
use sysinfo::Components;

#[derive(Debug, Clone, Serialize)]
pub struct ComponentTemp {
    pub label: String,
    pub temperature: f32,
    pub max: Option<f32>,
    pub critical: Option<f32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TemperatureInfo {
    pub components: Vec<ComponentTemp>,
}

impl TemperatureInfo {
    pub fn collect() -> Self {
        let components = Components::new_with_refreshed_list();

        let component_temps: Vec<ComponentTemp> = components
            .iter()
            .map(|component| ComponentTemp {
                label: component.label().to_string(),
                temperature: component.temperature(),
                max: component.max().into(),
                critical: component.critical(),
            })
            .collect();

        Self {
            components: component_temps,
        }
    }

    pub fn refresh(&mut self) {
        let mut components = Components::new_with_refreshed_list();
        components.refresh();

        self.components = components
            .iter()
            .map(|component| ComponentTemp {
                label: component.label().to_string(),
                temperature: component.temperature(),
                max: component.max().into(),
                critical: component.critical(),
            })
            .collect();
    }
}
