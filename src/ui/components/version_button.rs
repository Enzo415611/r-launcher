use iced::{Element, widget::button};

use crate::{events::events::Events, state::state::AppState};

impl AppState {
    pub fn version_button(&self) -> Element<'_, Events> {
        button("").into()
    }
}
