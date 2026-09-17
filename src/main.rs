#![windows_subsystem = "windows"]
mod ui;
mod service;
mod db;

use ui::app::App;

fn main() -> iced::Result {
    iced::daemon(App::new, App::update, App::view)
        .subscription(App::subscription)
        .run()
}