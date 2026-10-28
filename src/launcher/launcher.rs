use crate::state::instances::{self, Instance, to_loader};
use lighty_launcher::{
    Authenticator, Launch, Loader, VersionBuilder, auth, event::EventBus, version,
};

pub async fn launch_instance_off_account(
    instance: Instance,
    name: String,
    event_bus: EventBus,
) -> anyhow::Result<()> {
    let mut auth = auth::OfflineAuth::new(name);

    let profile = auth.authenticate(None).await?;

    let loader = instance
        .loader
        .clone()
        .unwrap_or(instances::Loader::Vanilla);
    let mc_version = instance.mc_version.clone().unwrap_or("26.3".to_string());

    let mut mc_instance: VersionBuilder<Loader> = version::VersionBuilder::new(
        &instance.name,
        to_loader(&loader.to_string()),
        &instance.loader_version,
        &mc_version,
    );

    mc_instance
        .launch(&profile, lighty_launcher::JavaDistribution::Temurin)
        .with_event_bus(&event_bus)
        .run()
        .await?;

    Ok(())
}
