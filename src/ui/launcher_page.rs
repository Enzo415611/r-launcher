use iced::{Element, widget::container};

use crate::{AppState, Events};

impl AppState {
    pub fn launcher_page(&self) -> Element<'_, Events> {
        container("content").into()
    }
}
