use iced::widget::space;
use iced::{window, Task};
use crate::ui::app::AppMessage;

pub enum MainWindowMessage {

}

pub enum MainWindowOutputMessage {

}

pub struct MainWindow {
    pub id: window::Id,
}

impl MainWindow {
    pub fn new(id: iced::window::Id) -> Self {
        Self {
            id
        }
    }

    pub fn view(&self) -> iced::Element<'_, MainWindowMessage> {
        space().into()
    }

    pub fn update(&mut self, message: MainWindowMessage) -> Task<MainWindowOutputMessage> {
        Task::none()
    }
}