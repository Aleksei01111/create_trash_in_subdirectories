use iced::Task;
use iced::window::Settings;
use crate::ui::main_window::{MainWindow, MainWindowMessage, MainWindowOutputMessage};
use iced::widget::{text, column, space};

pub enum AppMessage {
    MainWindowOpened(iced::window::Id),
    MainWindowMessages(MainWindowMessage),
    MainWindowOutput(MainWindowOutputMessage),
}

pub struct App {
    pub main_window: MainWindow,
}

impl App {
    pub fn new() -> (Self, Task<AppMessage>) {
        let (id, task) = iced::window::open(Settings::default());
        (
            Self {
                main_window: MainWindow::new(id)
            },

            task.map(AppMessage::MainWindowOpened)
            )
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::MainWindowOpened(id) => {
                Task::none()
            }

            AppMessage::MainWindowMessages(message) => {
                self.main_window.update(message).map(AppMessage::MainWindowOutput)
            }
        }
    }

    pub fn view(&self, window_id: iced::window::Id) -> iced::Element<'_, AppMessage> {
        if window_id == self.main_window.id {
            return self.main_window.view().map(AppMessage::MainWindowMessages);
        }

        space().into()
    }
}