use iced::{
    Element, Length,
    widget::{button, column, container, pick_list, row, rule, scrollable, text, text_input},
};

use crate::{
    events::{events::Events, instance_events::InstanceEvents},
    state::{instances::Loader, state::AppState},
};

const LOADER: [Loader; 6] = [
    Loader::Vanilla,
    Loader::Fabric,
    Loader::Forge,
    Loader::Optifine,
    Loader::NeoForge,
    Loader::Quilt,
];

impl AppState {
    pub fn instances(&self) -> Element<'_, Events> {
        let show_create_instance_button = button(text(format!("New Instance")))
            .on_press(Events::Instance(InstanceEvents::ShowCreateInstance));

        let scroll = scrollable(
            column![].extend(
                self.instances_state
                    .instances
                    .iter()
                    .map(|i| button(i.name.as_str()).into()),
            ),
        );

        container(
            row![
                column![show_create_instance_button, scroll],
                rule::vertical(0.8),
                self.create_instance()
            ]
            .spacing(3),
        )
        .into()
    }

    fn create_instance(&self) -> Option<Element<'_, Events>> {
        if self.instances_state.is_create_mode {
            let instance_name =
                text_input("Instance Name", &self.instances_state.create_instance.name)
                    .on_input(|n| Events::Instance(InstanceEvents::InputName(n)));

            let loader = pick_list(
                LOADER,
                self.instances_state.create_instance.loader.as_ref(),
                |loader| Events::Instance(InstanceEvents::SelectLoader(loader)),
            )
            .width(Length::Fill)
            .placeholder("Select Loader");

            let content = column![instance_name, loader, self.versions_button()].width(400);
            return Some(container(content).center(Length::Fill).into());
        }
        None
    }
}
// instance name
// loader
// loader version
// mc versions
