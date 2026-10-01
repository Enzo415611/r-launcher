use iced::{Element, widget::container};

use crate::{AppState, Events};

impl AppState {
    pub fn login_page(&self) -> Element<'_, Events> {
        container("login_page").into()
    }
}
