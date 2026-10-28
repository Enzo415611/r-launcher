use iced::{
    Alignment, Element, Length,
    widget::{button, column, container, pane_grid, pick_list, row, scrollable, text, text_input},
};

use crate::{
    events::{
        events::Events,
        instance_events::{
            create::CreateInstanceEvent, edit::EditInstanceEvent, instance_events::InstanceEvents,
        },
        ui_events::UiEvents,
    },
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
                            .clone()
                            .into_iter()
                            .map(|i| self.instance_button(i)),
                    ),
                );
                container(column![show_create_instance_button, scroll]).into()
            }
            InstancePane::Create => {
                container(column![self.create_instance(), self.edit_instance()]).into()
            }
        })
        .on_resize(50, |r| Events::Ui(UiEvents::InstancePanesResize(r)));

        pane_grid.into()
    }

    fn create_instance(&self) -> Option<Element<'_, Events>> {
        if self.instances_state.is_create_mode {
            let instance_name =
                text_input("Instance Name", &self.instances_state.create_instance.name).on_input(
                    |n| {
                        Events::Instance(InstanceEvents::CreateInstanceEvents(
                            CreateInstanceEvent::Name(n),
                        ))
                    },
                );

            let loader = pick_list(
                LOADER,
                self.instances_state.create_instance.loader.as_ref(),
                |l| {
                    Events::Instance(InstanceEvents::CreateInstanceEvents(
                        CreateInstanceEvent::Loader(l),
                    ))
                },
            )
            .width(Length::Fill)
            .placeholder("Select Loader");

            let loader_version: Option<Element<Events>> =
                if let Some(l) = &self.instances_state.create_instance.loader {
                    if !l.eq(&Loader::Vanilla) {
                        Some(
                            text_input(
                                "Loader Version",
                                &self.instances_state.create_instance.loader_version,
                            )
                            .on_input(|i| {
                                Events::Instance(InstanceEvents::CreateInstanceEvents(
                                    CreateInstanceEvent::LoaderVersion(i),
                                ))
                            })
                            .into(),
                        )
                    } else {
                        None
                    }
                } else {
                    None
                };

            let create = if !self.instances_state.create_instance.name.is_empty() {
                Some(button("Create").on_press(Events::Instance(
                    InstanceEvents::CreateInstanceEvents(CreateInstanceEvent::CreateInstance),
                )))
            } else {
                None
            };

            let content = column![
                instance_name,
                row![loader, loader_version].spacing(3),
                self.versions_button(false),
                create
            ]
            .align_x(Alignment::Center)
            .spacing(3)
            .width(400);
            return Some(container(content).center(Length::Fill).into());
        }
        None
    }

    pub fn edit_instance(&self) -> Option<Element<'_, Events>> {
        if self.instances_state.is_edit_mode {
            let instance_name =
                text_input("Instance Name", &self.instances_state.instance_edit.name).on_input(
                    |n| {
                        Events::Instance(InstanceEvents::EditInstanceEvents(
                            EditInstanceEvent::InputName(n),
                        ))
                    },
                );

            let loader = pick_list(
                LOADER,
                self.instances_state.instance_edit.loader.as_ref(),
                |loader| {
                    Events::Instance(InstanceEvents::EditInstanceEvents(
                        EditInstanceEvent::SelectLoader(loader),
                    ))
                },
            )
            .width(Length::Fill)
            .placeholder("Select Loader");

            let loader_version: Option<Element<Events>> =
                if let Some(l) = &self.instances_state.instance_edit.loader {
                    if !l.eq(&Loader::Vanilla) {
                        Some(
                            text_input(
                                "Loader Version",
                                &self.instances_state.instance_edit.loader_version,
                            )
                            .on_input(|v| {
                                Events::Instance(InstanceEvents::EditInstanceEvents(
                                    EditInstanceEvent::LoaderVersion(v),
                                ))
                            })
                            .into(),
                        )
                    } else {
                        None
                    }
                } else {
                    None
                };

            let confirm_button =
                if self.instances_state.instance_edit != self.instances_state.instance_selected {
                    Some(button("Confirm").on_press(Events::Instance(
                        InstanceEvents::EditInstanceEvents(EditInstanceEvent::Confirm),
                    )))
                } else {
                    None
                };

            return Some(
                container(column![
                    instance_name,
                    row![loader, loader_version],
                    self.versions_button(true),
                    confirm_button
                ])
                .into(),
            );
        }
        None
    }
}
