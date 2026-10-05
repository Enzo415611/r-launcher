use iced::{Element, widget::container};

use crate::{events::events::Events, state::state::AppState};

impl AppState {
    pub fn launcher_page(&self) -> Element<'_, Events> {
        container("content").into()
    }
}
