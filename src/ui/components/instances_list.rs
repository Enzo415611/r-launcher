use iced::{
    Element,
    widget::{button, column, container, scrollable},
};

use crate::{
    events::{events::Events, instance_events::InstanceEvents},
    state::state::AppState,
};

impl AppState {
    pub fn instances(&self) -> Element<'_, Events> {
        let create_instance_button =
            button("New").on_press(Events::Instance(InstanceEvents::ShowCreateInstance));
        let scroll = scrollable(
            column![].extend(
                self.instances_state
                    .instances
                    .iter()
                    .map(|i| button(i.name.as_str()).into()),
            ),
        );
        container(column![create_instance_button, scroll]).into()
    }

    fn create_instance(&self) -> Option<Element<'_, Events>> {
        if self.instances_state.is_create_mode {
            let content = column![];
            return Some(container(content).into());
        }
        None
    }
}
