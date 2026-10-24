use iced::Task;

use crate::{
    events::events::Events,
    state::{instances::Loader, state::AppState},
};

#[derive(Clone, Debug)]
pub enum InstanceEvents {
    ShowCreateInstance,
    SelectLoader(Loader),
    InputName(String),
    CreateInstance,
    GetLoaderVersions,
}

impl AppState {
    pub fn instance_events(&mut self, e: InstanceEvents) -> Task<Events> {
        match e {
            InstanceEvents::CreateInstance => Task::none(),
            InstanceEvents::ShowCreateInstance => {
                self.instances_state.is_create_mode = !self.instances_state.is_create_mode;
                Task::none()
            }
            InstanceEvents::SelectLoader(l) => {
                self.instances_state.create_instance.loader = Some(l);
                Task::none()
            }
            InstanceEvents::GetLoaderVersions => Task::none(),
            InstanceEvents::InputName(name) => {
                self.instances_state.create_instance.name = name;
                Task::none()
            }
        }
    }
}
