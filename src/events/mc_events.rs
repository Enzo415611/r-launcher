use iced::Task;
use lighty_launcher::event::EventBus;

use crate::{
    events::{events::Events, launcher_events::LauncherEvents},
    launcher::launcher::launch_instance_off_account,
    state::state::AppState,
};

#[derive(Clone, Debug)]

pub enum McEvents {
    LaunchInstance,
    LaunchInstanceFuture,
    EventBus,
}

impl AppState {
    pub fn mc_events(&mut self, e: McEvents) -> Task<Events> {
        match e {
            McEvents::LaunchInstance => {
                if let Some(u) = &self.launcher_state.current_account {
                    let instance = self.instances_state.instance_selected.clone();

                    return Task::perform(
                        launch_instance_off_account(
                            instance,
                            u.username.clone(),
                            self.launcher_state.event_bus.clone(),
                        ),
                        |r| match r {
                            Ok(_) => Events::Launcher(LauncherEvents::McEvent(
                                McEvents::LaunchInstanceFuture,
                            )),
                            Err(err) => Events::ErrMsg(err.to_string()),
                        },
                    );
                }
                Task::none()
            }
            McEvents::LaunchInstanceFuture => Task::perform(
                mc_instance_event_bus(self.launcher_state.event_bus.clone()),
                |_| Events::Launcher(LauncherEvents::McEvent(McEvents::EventBus)),
            ),
            McEvents::EventBus => Task::none(),
        }
    }
}

pub async fn mc_instance_event_bus(event_bus: EventBus) {
    let mut receiver = event_bus.subscribe();
    while let Ok(event) = receiver.next().await {
        println!("event");
        match event {
            lighty_launcher::event::Event::InstanceExited(_) => {
                break;
            }
            _ => {}
        }
    }
}
