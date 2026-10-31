use crate::events::{events::Events, login_events::LoginEvents};
use iced::{
    Subscription,
    futures::{SinkExt, Stream, channel::mpsc},
    stream,
};
use lighty_launcher::auth::{ExposeSecret, MicrosoftAuth, SecretString};

const SERVICE: &str = "RLauncher";
const USERNAME: &str = "microsoft_refresh_token";

fn load_refresh_token() -> Option<SecretString> {
    let entry = keyring::Entry::new(SERVICE, USERNAME).ok()?;
    let token = entry.get_password().ok()?;
    Some(SecretString::from(token))
}

fn save_refresh_code(token: &SecretString) -> anyhow::Result<()> {
    let entry = keyring::Entry::new(SERVICE, USERNAME)?;
    entry.set_password(token.expose_secret())?;
    Ok(())
}

async fn online_login() {
    let mut auth = MicrosoftAuth::new("");
    Subscription::run(listen_to_auth_callback);
}

fn listen_to_auth_callback() -> impl Stream<Item = Events> {
    stream::channel(100, async |mut out| {
        let (tx, mut rx) = mpsc::channel::<(String, String)>(10);

        tokio::task::spawn_blocking(move || {
            let mut auth = MicrosoftAuth::new("");
            auth.set_device_code_callback(move |code, url| {
                tx.clone().send((code.to_string(), url.to_string()));
            });
        });

        while let Ok((code, url)) = rx.recv().await {
            let _ = out.send(Events::Login(LoginEvents::DeviceCodeReceived { code, url }));
        }
    })
}
