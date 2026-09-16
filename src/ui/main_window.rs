use std::collections::HashMap;
use iced::widget::{text, column, text_input, button, container, scrollable, row, space, checkbox};
use iced::{window, Task, Element};
use iced::widget::scrollable::Direction;
use crate::service::files_creator::FilesCreator;

#[derive(Debug, Clone)]
pub enum MainWindowMessage {
    PathToDirectory(String),

    StartGenerateFiles,

    FileName(String),
    Depth(String),

    AddContentVariant,

    ContentVariantEdit(i32, String),
    ContentVariantDelete(i32),

    CloseAsDone(bool),

    FileCreationDelay(String),
}

#[derive(PartialEq)]
pub enum MainWindowOutputMessage {
    Close,
}

pub struct MainWindow {
    pub id: window::Id,

    pub path_to_directory_input: String,

    pub depth_str: String,
    pub depth: i32,

    pub close_as_done: bool,

    pub file_creation_delay_in_milliseconds_str: String,

    pub files_content_variants: HashMap<i32, String>,

    files_creator: FilesCreator,
}

impl MainWindow {
    pub fn new(id: window::Id) -> Self {
        Self {
            id,
            path_to_directory_input: String::new(),
            files_creator: FilesCreator::new(),
            depth_str: String::new(),
            file_creation_delay_in_milliseconds_str: String::new(),
            files_content_variants: HashMap::new(),
            depth: 0,
            close_as_done: false,
        }
    }

    pub fn view(&self) -> iced::Element<'_, MainWindowMessage> {
        container(row![
            self.let_side(),
            space().width(20),
            self.right_side(),
        ])
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
                self.files_creator.create_files(self.path_to_directory_input.clone(), self.depth, 0, &self.files_content_variants);

                if self.close_as_done {
                    return Task::done(MainWindowOutputMessage::Close)
                }

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
            MainWindowMessage::AddContentVariant => {
                let id = self.files_content_variants.len() as i32;
                self.files_content_variants.insert(id, String::new());
                Task::none()
            }
            MainWindowMessage::ContentVariantEdit(id, content) => {
                if let Some(value) = self.files_content_variants.get_mut(&id) {
                    *value = content;
                }

                Task::none()
            }
            MainWindowMessage::ContentVariantDelete(id) => {
                self.files_content_variants.remove(&id);
                Task::none()
            }
            MainWindowMessage::CloseAsDone(new_value) => {
                self.close_as_done = new_value;
                Task::none()
            }
            MainWindowMessage::FileCreationDelay(new_value) => {
                let parsed = new_value.parse::<u64>();
                if !parsed.is_err() {
                    self.files_creator.file_creation_delay_in_milliseconds = parsed.unwrap();
                    self.file_creation_delay_in_milliseconds_str = new_value;
                }
                Task::none()
            }
        }
    }

    fn right_side(&self) -> Element<'_, MainWindowMessage> {
        let content_variants = self.files_content_variants
            .iter()
            .map(|(id, content_variant)|
                {
                    row![
                        text_input("содержание", content_variant).on_input(move |val| {MainWindowMessage::ContentVariantEdit(*id, val)}),
                        button("Удалить").on_press(MainWindowMessage::ContentVariantDelete(*id)),
                    ].into()
                });

        column![
            text("Варианты для содержания создаваемых файлов"),
            button("Добавить").on_press(MainWindowMessage::AddContentVariant),
            scrollable(
                column(content_variants).spacing(5),
            ),
        ]
            .spacing(10)
            .into()
    }

    fn let_side(&self) -> iced::Element<'_, MainWindowMessage> {
        column![
            text_input("Путь до папки", &self.path_to_directory_input).on_input(MainWindowMessage::PathToDirectory),
            text_input("Имя для каждого файла", &self.files_creator.filename).on_input(MainWindowMessage::FileName),
            text_input("Глубина", &self.depth_str).on_input(MainWindowMessage::Depth),
            checkbox(self.close_as_done).label("Закрыть по завершении").on_toggle(MainWindowMessage::CloseAsDone),
            text_input("Задержка между созданием файла (мс)", &self.file_creation_delay_in_milliseconds_str).on_input(MainWindowMessage::FileCreationDelay),
            button("Начать").on_press(MainWindowMessage::StartGenerateFiles),
            self.scrollable_text_output()
        ].spacing(10).into()
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