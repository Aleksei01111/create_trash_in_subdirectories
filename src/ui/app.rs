use iced::{window, Subscription, Task};
use iced::window::Settings;
use crate::ui::main_window::{MainWindow, MainWindowMessage, MainWindowOutputMessage};
use iced::widget::{space};

pub enum AppMessage {
    MainWindowOpened(window::Id),
    MainWindowMessages(MainWindowMessage),
    MainWindowOutput(MainWindowOutputMessage),

    WindowClosed(window::Id)
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
            AppMessage::MainWindowOpened(_) => {
                Task::none()
            }

            AppMessage::MainWindowMessages(message) => {
                self.main_window.update(message).map(AppMessage::MainWindowOutput)
            }

            AppMessage::WindowClosed(id) => {
                if id == self.main_window.id {
                    return iced::exit()
                }

                Task::none()
            }
            AppMessage::MainWindowOutput(out) => {
                if out == MainWindowOutputMessage::Close {
                    return iced::exit()
                }

                Task::none()
            }
        }
    }

    pub fn view(&self, window_id: iced::window::Id) -> iced::Element<'_, AppMessage> {
        if window_id == self.main_window.id {
            return self.main_window.view().map(AppMessage::MainWindowMessages);
        }

        space().into()
    }

    pub fn subscription(&self) -> Subscription<AppMessage> {
        window::close_events().map(AppMessage::WindowClosed)
    }
}