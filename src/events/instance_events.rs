use iced::Task;

use crate::{
    db::{dto::instances_dto::InstanceDto, service::instances_service},
    events::events::Events,
    state::{
        instances::{Instance, Loader, to_loader},
        state::AppState,
    },
};

#[derive(Clone, Debug)]
pub enum InstanceEvents {
    ShowCreateInstance,
    SelectLoader(Loader),
    InputName(String),
    CreateInstance,
    CreateInstanceFuture(Instance),
    UpdateInstance,
    RemoveInstance,
    GetLoaderVersions,
    InstanceSelected,
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
                        version: Some(i.version),
                        loader: Some(to_loader(&i.loader)),
                    })
                    .collect();
                Task::none()
            }
            InstanceEvents::CreateInstance => {
                let new_instance = self.instances_state.create_instance.clone();
                self.instances_state.create_instance.name.clear();
                if let Some(db) = self.db.clone() {
                    let new_instance = InstanceDto {
                        id: new_instance.id,
                        name: new_instance.name,
                        loader: new_instance.loader.unwrap_or(Loader::Vanilla).to_string(),
                        version: new_instance.version.unwrap_or("26.3".to_string()),
                    };
                    return Task::perform(
                        instances_service::save_instance(db, new_instance),
                        |r| match r {
                            Ok(i) => {
                                Events::Instance(InstanceEvents::CreateInstanceFuture(Instance {
                                    id: i.id,
                                    name: i.name,
                                    version: Some(i.version),
                                    loader: Some(to_loader(&i.loader)),
                                }))
                            }
                            Err(err) => Events::ErrMsg(err.to_string()),
                        },
                    );
                }
                Task::none()
            }
            InstanceEvents::CreateInstanceFuture(instance) => {
                self.instances_state.instances.push(instance);
                Task::none()
            }
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
            InstanceEvents::InstanceSelected => Task::none(),
            InstanceEvents::RemoveInstance => Task::none(),
            InstanceEvents::UpdateInstance => Task::none(),
        }
    }
}
