use iced::{
    Alignment, Element, Length,
    widget::{button, column, container, pane_grid, pick_list, scrollable, text, text_input},
};

use crate::{
    events::{events::Events, instance_events::InstanceEvents, ui_events::UiEvents},
    state::{instances::Loader, state::AppState, ui_state::InstancePane},
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
        let pane_grid = pane_grid(&self.ui_state.instances_pane, |_, panes, _| match panes {
            InstancePane::List => {
                let show_create_instance_button = button(text(format!("New Instance")))
                    .on_press(Events::Instance(InstanceEvents::ShowCreateInstance));

                let scroll = scrollable(
                    column![].extend(
                        self.instances_state
                            .instances
                            .iter()
                            .map(|i| self.instance_button(i.name.clone())),
                    ),
                );
                container(column![show_create_instance_button, scroll]).into()
            }
            InstancePane::Create => container(self.create_instance()).into(),
        })
        .on_resize(50, |r| Events::Ui(UiEvents::InstancePanesResize(r)));

        pane_grid.into()
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

            let create = if !self.instances_state.create_instance.name.is_empty() {
                Some(button("Create").on_press(Events::Instance(InstanceEvents::CreateInstance)))
            } else {
                None
            };

            let content = column![instance_name, loader, self.versions_button(), create]
                .align_x(Alignment::Center)
                .spacing(3)
                .width(400);
            return Some(container(content).center(Length::Fill).into());
        }
        None
    }
}
