use iced::Task;

use crate::{events::events::Events, state::state::AppState};

#[derive(Clone, Debug)]
pub enum InstanceEvents {
    ShowCreateInstance,
    CreateInstance,
    GetLoaderVersions,
}

impl AppState {
    pub fn instance_events(&mut self, e: InstanceEvents) -> Task<Events> {
        match e {
            InstanceEvents::ShowCreateInstance => {
                self.instances_state.is_create_mode = !self.instances_state.is_create_mode;
                Task::none()
            }
            InstanceEvents::CreateInstance => Task::none(),
            InstanceEvents::GetLoaderVersions => Task::none(),
        }
    }
}
