mod ui;

use ui::app::App;

fn main() -> iced::Result {
    iced::daemon(App::new, App::update, App::view)
        .run()
}
