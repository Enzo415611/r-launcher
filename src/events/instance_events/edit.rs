use iced::Task;

use crate::{
    db::{dto::instances_dto::InstanceDto, service::instances_service::update_instance},
    events::{events::Events, instance_events::instance_events::InstanceEvents},
    state::{instances::Loader, state::AppState},
};

#[derive(Clone, Debug)]
pub enum EditInstanceEvent {
    Confirm,
    InputName(String),
    SelectLoader(Loader),
    Version(String),
    LoaderVersion(String),
}

impl AppState {
    pub fn edit_instance_events(&mut self, e: EditInstanceEvent) -> Task<Events> {
        match e {
            EditInstanceEvent::InputName(n) => {
                self.instances_state.instance_edit.name = n;
            }
            EditInstanceEvent::SelectLoader(l) => {
                self.instances_state.instance_selected.loader = Some(l);
            }
            EditInstanceEvent::LoaderVersion(v) => {
                self.instances_state.instance_edit.loader_version = v;
            }
            EditInstanceEvent::Version(v) => {
                self.instances_state.instance_edit.mc_version = Some(v)
            }
            EditInstanceEvent::Confirm => {
                if let Some(db) = self.db.clone() {
                    let id = self.instances_state.instance_edit.id;
                    let instance_edit = self.instances_state.instance_edit.clone();

                    let instance = InstanceDto {
                        id: 0,
                        name: instance_edit.name,
                        version: instance_edit.mc_version.unwrap_or("26.3".to_string()),
                        loader_version: instance_edit.loader_version,
                        loader: instance_edit.loader.unwrap_or(Loader::Vanilla).to_string(),
                    };
                    return Task::perform(update_instance(db, instance, id), |r| match r {
                        Ok(i) => Events::Instance(InstanceEvents::UpdateInstance(i)),
                        Err(err) => Events::ErrMsg(err.to_string()),
                    });
                }
            }
        }
        Task::none()
    }
}
