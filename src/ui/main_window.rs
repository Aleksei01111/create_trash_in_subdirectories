use iced::widget::{text, column, text_input, button, container, scrollable, row, space, checkbox};
use iced::{window, Task, Element};
use iced::widget::scrollable::Direction;
use crate::service::files_creator::{FilesCreator};
use crate::service::files_creator_configuration::{FilesCreatorConfiguration, FilesCreatorConfigurationSaver};

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

    FileCreationDelayLowLimit(String),
    FileCreationDelayHighLimit(String),

    ContainExclusionVariantEdit(i32, String),
    ContainExclusionVariantRemove(i32),
    AddContainExclusionVariant,
    ContainExclusionsIgnoreCaseToggle(bool),

    SaveConfigurationToFile,
    LoadConfigurationFromFile,

    NotCreateFilesEdited(bool),
}

#[derive(PartialEq)]
pub enum MainWindowOutputMessage {
    Close,
}

pub struct MainWindow {
    pub id: window::Id,

    depth_str: String,
    file_creation_delay_in_milliseconds_low_limit_str: String,
    file_creation_delay_in_milliseconds_high_limit_str: String,

    pub close_as_done: bool,

    files_creator: FilesCreator,
    files_creator_configuration: FilesCreatorConfiguration,

    files_creator_configuration_path: String,

    not_create_files: bool,
}

impl MainWindow {
    pub fn new(id: window::Id) -> Self {
        Self {
            id,
            files_creator: FilesCreator::new(),
            depth_str: String::new(),
            file_creation_delay_in_milliseconds_low_limit_str: String::new(),
            file_creation_delay_in_milliseconds_high_limit_str: String::new(),
            close_as_done: false,
            files_creator_configuration: FilesCreatorConfiguration::new(String::from(""), 100, 500, String::from(""), 0),
            files_creator_configuration_path: String::from("config.lenovo"),
            not_create_files: false,
        }
    }

    pub fn view(&self) -> iced::Element<'_, MainWindowMessage> {
        container(row![
            self.let_side(),
            space().width(20),
            self.right_side(),
        ])
            .padding(15)
            .into()
    }

    pub fn update(&mut self, message: MainWindowMessage) -> Task<MainWindowOutputMessage> {
        match message {
            MainWindowMessage::PathToDirectory(new_value) => {
                self.files_creator_configuration.path_to_directory = new_value;
                Task::none()
            },
            MainWindowMessage::StartGenerateFiles => {
                self.files_creator.out_string = String::new();
                self.files_creator.create_files(0, self.files_creator_configuration.path_to_directory.clone(), &self.files_creator_configuration, self.not_create_files);

                if self.close_as_done {
                    return Task::done(MainWindowOutputMessage::Close)
                }

                Task::none()
            }
            MainWindowMessage::FileName(text) => {
                self.files_creator_configuration.filename = text;
                Task::none()
            }
            MainWindowMessage::Depth(text) => {
                let parsed = text.parse::<i32>();
                if !parsed.is_err() {
                    self.files_creator_configuration.depth = parsed.unwrap();
                    self.depth_str = text;
                }
                Task::none()
            }
            MainWindowMessage::AddContentVariant => {
                let id = self.files_creator_configuration.content_variants.len() as i32;
                self.files_creator_configuration.content_variants.insert(id, String::new());
                Task::none()
            }
            MainWindowMessage::ContentVariantEdit(id, content) => {
                if let Some(value) = self.files_creator_configuration.content_variants.get_mut(&id) {
                    *value = content;
                }

                Task::none()
            }
            MainWindowMessage::ContentVariantDelete(id) => {
                self.files_creator_configuration.content_variants.remove(&id);
                Task::none()
            }
            MainWindowMessage::CloseAsDone(new_value) => {
                self.close_as_done = new_value;
                Task::none()
            }
            MainWindowMessage::FileCreationDelayLowLimit(new_value) => {
                let parsed = new_value.parse::<u64>();
                if !parsed.is_err() {
                    self.files_creator_configuration.file_creation_delay_in_milliseconds_low_limit = parsed.unwrap();
                    self.file_creation_delay_in_milliseconds_low_limit_str = new_value;
                }
                Task::none()
            }
            MainWindowMessage::FileCreationDelayHighLimit(new_value) => {
                let parsed = new_value.parse::<u64>();
                if !parsed.is_err() {
                    self.files_creator_configuration.file_creation_delay_in_milliseconds_high_limit = parsed.unwrap();
                    self.file_creation_delay_in_milliseconds_high_limit_str = new_value;
                }
                Task::none()
            }
            MainWindowMessage::ContainExclusionVariantEdit(id, new_value) => {
                if new_value.len() == 0 {
                    self.files_creator_configuration.contain_exclusions.remove(&id);
                }
                else if let Some(value) = self.files_creator_configuration.contain_exclusions.get_mut(&id) {
                    *value = new_value;
                }

                Task::none()
            }
            MainWindowMessage::ContainExclusionVariantRemove(id) => {
                self.files_creator_configuration.contain_exclusions.remove(&id);
                Task::none()
            }
            MainWindowMessage::AddContainExclusionVariant => {
                let id = self.files_creator_configuration.contain_exclusions.len() as i32;
                self.files_creator_configuration.contain_exclusions.insert(id, String::new());

                Task::none()
            }
            MainWindowMessage::ContainExclusionsIgnoreCaseToggle(new_value) => {
                self.files_creator_configuration.contain_exclusions_ignore_case = new_value;
                Task::none()
            }
            MainWindowMessage::SaveConfigurationToFile => {
                let _ = FilesCreatorConfigurationSaver::save_to_file(&self.files_creator_configuration_path, &self.files_creator_configuration);

                Task::none()
            }
            MainWindowMessage::LoadConfigurationFromFile => {
                let result = FilesCreatorConfigurationSaver::load_from_file(&self.files_creator_configuration_path);

                if result.is_ok() {
                    self.files_creator_configuration = result.unwrap();
                    self.depth_str = self.files_creator_configuration.depth.to_string();
                    self.file_creation_delay_in_milliseconds_low_limit_str =
                        self.files_creator_configuration.file_creation_delay_in_milliseconds_low_limit.to_string();
                    self.file_creation_delay_in_milliseconds_high_limit_str =
                        self.files_creator_configuration.file_creation_delay_in_milliseconds_high_limit.to_string();
                }

                Task::none()
            }
            MainWindowMessage::NotCreateFilesEdited(new_value) => {
                self.not_create_files = new_value;
                Task::none()
            }
        }
    }

    fn right_side(&self) -> Element<'_, MainWindowMessage> {
        let content_variants = self.files_creator_configuration.content_variants
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

    fn let_side(&self) -> Element<'_, MainWindowMessage> {
        let content = column![
            checkbox(self.not_create_files)
                .label("Не создавать файлы")
                .on_toggle(MainWindowMessage::NotCreateFilesEdited),
            checkbox(self.close_as_done)
                .label("Закрыть по завершении")
                .on_toggle(MainWindowMessage::CloseAsDone),

            space().height(20),

            text_input("Путь до папки", &self.files_creator_configuration.path_to_directory).on_input(MainWindowMessage::PathToDirectory),
            text_input("Имя для каждого файла", &self.files_creator_configuration.filename).on_input(MainWindowMessage::FileName),
            text_input("Глубина", &self.depth_str).on_input(MainWindowMessage::Depth),

            space().height(20),

            text("Задержка между созданием файла (мс)"),

            row![
                text_input("нижняя граница",
                    &self.file_creation_delay_in_milliseconds_low_limit_str).on_input(MainWindowMessage::FileCreationDelayLowLimit),
                text_input("верхняя граница",
                    &self.file_creation_delay_in_milliseconds_high_limit_str).on_input(MainWindowMessage::FileCreationDelayHighLimit),
            ].spacing(10),

            space().height(20),

            self.directory_contain_exclusions(),

            button("Начать").on_press(MainWindowMessage::StartGenerateFiles),

            space().height(20),

            row![
                button("Загрузить конфигурацию").on_press(MainWindowMessage::LoadConfigurationFromFile),
                button("Сохранить конфигурацию").on_press(MainWindowMessage::SaveConfigurationToFile),
            ].spacing(15),

            self.scrollable_text_output(),
        ].spacing(10);

        scrollable(
            content
        ).into()
    }

    fn directory_contain_exclusions(&self) -> Element<'_, MainWindowMessage> {
        let content = self.files_creator_configuration.contain_exclusions.iter().map(|(id, string)| {
            row![
                text_input("исключение", string).on_input(move |val| {MainWindowMessage::ContainExclusionVariantEdit(*id, val)}),
                button("Удалить").on_press(MainWindowMessage::ContainExclusionVariantRemove(*id)),
            ].into()
        });

        column![
            row![
                text("Исключения (путь должен содержать)"),
                button("Добавить").on_press(MainWindowMessage::AddContainExclusionVariant)
            ].spacing(10),

            checkbox(self.files_creator_configuration.contain_exclusions_ignore_case)
                .label("Игнорировать регистр")
                .on_toggle(MainWindowMessage::ContainExclusionsIgnoreCaseToggle),

            scrollable(column(content))
        ].into()
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