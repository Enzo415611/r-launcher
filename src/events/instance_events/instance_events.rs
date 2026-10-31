use iced::Task;

use crate::{
    db::{
        dto::instances_dto::InstanceDto,
        service::instances_service::{self, remove_instance},
    },
    events::{
        events::Events,
        instance_events::{create::CreateInstanceEvent, edit::EditInstanceEvent},
        launcher_events::LauncherEvents,
    },
    state::{
        instances::{self, Instance},
        state::AppState,
    },
};

#[derive(Clone, Debug)]
pub enum InstanceEvents {
    ShowCreateInstance,
    ShowEditInstance(Instance),
    UpdateInstance(InstanceDto),
    CreateInstanceEvents(CreateInstanceEvent),
    EditInstanceEvents(EditInstanceEvent),
    RemoveInstance(Instance),
    RemoveInstanceFuture,
    InstanceSelected(Instance),
    GetAll,
    GetAllFuture(Vec<InstanceDto>),
}

impl AppState {
    pub fn instance_events(&mut self, e: InstanceEvents) -> Task<Events> {
        match e {
            InstanceEvents::GetAll => {
                if let Some(db) = self.db.clone() {
                    return Task::perform(instances_service::get_all(db), |r| match r {
                        Ok(v) => Events::Instance(InstanceEvents::GetAllFuture(v)),
                        Err(err) => Events::ErrMsg(err.to_string()),
                    });
                }
                Task::none()
            }
            InstanceEvents::GetAllFuture(all) => {
                self.instances_state.instances = all
                    .into_iter()
                    .map(|i| Instance {
                        id: i.id,
                        name: i.name,
                        mc_version: Some(i.version),
                        loader_version: i.loader_version,
                        loader: Some(instances::Loader::to_loader(&i.loader)),
                    })
                    .collect();
                Task::none()
            }
            InstanceEvents::CreateInstanceEvents(e) => self.create_instance_events(e),
            InstanceEvents::ShowCreateInstance => {
                self.instances_state.is_create_mode = true;
                self.instances_state.is_edit_mode = false;
                Task::none()
            }
            InstanceEvents::ShowEditInstance(instance) => {
                self.instances_state.is_create_mode = false;
                self.instances_state.is_edit_mode = true;
                self.instances_state.instance_edit = instance.clone();
                self.instances_state.instance_selected = instance;
                Task::none()
            }
            InstanceEvents::InstanceSelected(instance) => {
                self.instances_state.instance_selected = instance;
                Task::done(Events::Launcher(LauncherEvents::McEvent(
                    crate::events::mc_events::McEvents::LaunchInstance,
                )))
            }
            InstanceEvents::RemoveInstance(instance) => {
                if self.instances_state.instances.shift_remove(&instance) {
                    if let Some(db) = self.db.clone() {
                        return Task::perform(remove_instance(db, instance.id), |r| match r {
                            Ok(_) => Events::Instance(InstanceEvents::RemoveInstanceFuture),
                            Err(err) => Events::ErrMsg(err.to_string()),
                        });
                    }
                }
                Task::none()
            }
            InstanceEvents::RemoveInstanceFuture => Task::none(),
            InstanceEvents::UpdateInstance(i) => {
                let instance_update = Instance {
                    id: i.id,
                    name: i.name,
                    loader: Some(instances::Loader::to_loader(i.loader.as_str())),
                    loader_version: i.loader_version,
                    mc_version: Some(i.version),
                };
                self.instances_state.is_edit_mode = false;

                if let Some(i) = self
                    .instances_state
                    .instances
                    .get_index_of(&self.instances_state.instance_selected)
                {
                    _ = self
                        .instances_state
                        .instances
                        .replace_index(i, instance_update);
                }

                Task::none()
            }
            InstanceEvents::EditInstanceEvents(e) => self.edit_instance_events(e),
        }
    }
}
