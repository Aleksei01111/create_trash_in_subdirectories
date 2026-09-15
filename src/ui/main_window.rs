use iced::widget::{text, column, text_input, button, container, scrollable};
use iced::{window, Task, Element};
use iced::widget::scrollable::Direction;
use crate::FilesCreator;

#[derive(Debug, Clone)]
pub enum MainWindowMessage {
    PathToDirectory(String),

    StartGenerateFiles,

    FileName(String),
    Depth(String),
}

pub enum MainWindowOutputMessage {

}

pub struct MainWindow {
    pub id: window::Id,

    pub path_to_directory_input: String,
    pub depth_str: String,
    pub depth: i32,

    files_creator: FilesCreator,
}

impl MainWindow {
    pub fn new(id: window::Id) -> Self {
        Self {
            id,
            path_to_directory_input: String::new(),
            files_creator: FilesCreator::new(),
            depth_str: String::new(),
            depth: 0,
        }
    }

    pub fn view(&self) -> iced::Element<'_, MainWindowMessage> {
        container(column![
            text_input("Путь до папки", &self.path_to_directory_input).on_input(MainWindowMessage::PathToDirectory),
            text_input("Имя для каждого файла", &self.files_creator.filename).on_input(MainWindowMessage::FileName),
            text_input("Глубина", &self.depth_str).on_input(MainWindowMessage::Depth),
            button("Начать").on_press(MainWindowMessage::StartGenerateFiles),
            self.scrollable_text_output()
        ].max_width(600).spacing(10))
            .center_x(iced::Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: MainWindowMessage) -> Task<MainWindowOutputMessage> {
        match message {
            MainWindowMessage::PathToDirectory(new_value) => {
                self.path_to_directory_input = new_value;
                Task::none()
            },
            MainWindowMessage::StartGenerateFiles => {
                self.files_creator.out_string = String::new();
                self.files_creator.create_files(self.path_to_directory_input.clone(), self.depth, 0);
                Task::none()
            }
            MainWindowMessage::FileName(text) => {
                self.files_creator.filename = text;
                Task::none()
            }
            MainWindowMessage::Depth(text) => {
                let parsed = text.parse::<i32>();
                if !parsed.is_err() {
                    self.depth = parsed.unwrap();
                    self.depth_str = text;
                }
                Task::none()
            }
        }
    }

    fn scrollable_text_output(&self) -> Element<'_, MainWindowMessage> {
        column![
            scrollable(container(text(&self.files_creator.out_string)).padding(10)).direction({
                let scrollbar = scrollable::Scrollbar::new();

                Direction::Both {
                    horizontal: scrollbar,
                    vertical: scrollbar,
                }
            })
        ].into()
    }
}