use iced::{
    Element,
    widget::{button, container, row, text},
};

use crate::{
    events::{events::Events, instance_events::InstanceEvents},
    state::state::AppState,
};

impl AppState {
    pub fn instance_button(&self, name: String) -> Element<Events> {
        container(row![
            button("Edit").on_press(Events::Instance(InstanceEvents::UpdateInstance)),
            button(text(name)).on_press(Events::Instance(InstanceEvents::InstanceSelected)),
            button("X").on_press(Events::Instance(InstanceEvents::RemoveInstance))
        ])
        .into()
    }
}
