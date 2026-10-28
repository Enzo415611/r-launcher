use iced::Task;

use crate::{
    db::{dto::instances_dto::InstanceDto, service::instances_service},
    events::{events::Events, instance_events::instance_events::InstanceEvents},
    state::{
        instances::{Instance, Loader},
        state::AppState,
    },
};

#[derive(Clone, Debug)]
pub enum CreateInstanceEvent {
    CreateInstance,
    Name(String),
    Version(String),
    Loader(Loader),
    LoaderVersion(String),
    CreateInstanceFuture(Instance),
}

impl AppState {
    pub fn create_instance_events(&mut self, e: CreateInstanceEvent) -> Task<Events> {
        match e {
            CreateInstanceEvent::CreateInstance => {
                let new_instance = self.instances_state.create_instance.clone();
                self.instances_state.create_instance.name.clear();
                self.instances_state.create_instance.mc_version = self
                    .launcher_state
                    .version_state
                    .release_versions_vec
                    .first()
                    .cloned();

                if let Some(db) = self.db.clone() {
                    let new_instance_dto = InstanceDto {
                        id: new_instance.id,
                        name: new_instance.name,
                        loader: new_instance.loader.unwrap_or(Loader::Vanilla).to_string(),
                        version: new_instance.mc_version.unwrap_or("26.3".to_string()),
                        loader_version: new_instance.loader_version,
                    };
                    return Task::perform(
                        instances_service::save_instance(db, new_instance_dto),
                        |r| match r {
                            Ok(i) => Events::Instance(InstanceEvents::CreateInstanceEvents(
                                CreateInstanceEvent::CreateInstanceFuture(Instance {
                                    id: i.id,
                                    name: i.name,
                                    loader: Some(Loader::to_loader(&i.loader)),
                                    loader_version: i.loader_version,
                                    mc_version: Some(i.version),
                                }),
                            )),
                            Err(err) => Events::ErrMsg(err.to_string()),
                        },
                    );
                }
                Task::none()
            }
            CreateInstanceEvent::Version(v) => {
                self.instances_state.create_instance.mc_version = Some(v);
                Task::none()
            }
            CreateInstanceEvent::CreateInstanceFuture(instance) => {
                self.instances_state.instances.insert(instance);
                self.instances_state.is_create_mode = false;
                Task::none()
            }
            CreateInstanceEvent::Loader(l) => {
                self.instances_state.create_instance.loader = Some(l);
                Task::none()
            }
            CreateInstanceEvent::LoaderVersion(v) => {
                self.instances_state.create_instance.loader_version = v;
                Task::none()
            }
            CreateInstanceEvent::Name(name) => {
                self.instances_state.create_instance.name = name;
                Task::none()
            }
        }
    }
}
