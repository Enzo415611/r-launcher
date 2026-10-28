use iced::{
    Element,
    widget::{button, container, row, text},
};

use crate::{
    events::{events::Events, instance_events::instance_events::InstanceEvents},
    state::{instances::Instance, state::AppState},
};

impl AppState {
    pub fn instance_button(&self, instance: Instance) -> Element<'_, Events> {
        container(
            row![
                button("Edit").on_press(Events::Instance(InstanceEvents::ShowEditInstance(
                    instance.clone()
                ))),
                button(text(instance.name.clone())).on_press(Events::Instance(
                    InstanceEvents::InstanceSelected(instance.clone())
                )),
                button("X").on_press(Events::Instance(InstanceEvents::RemoveInstance(instance)))
            ]
            .spacing(3.0),
        )
        .into()
    }
}
