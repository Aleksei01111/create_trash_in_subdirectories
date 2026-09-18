use iced::{Background, Color, Element};
use iced::widget::{button, center, container, mouse_area, opaque, stack, text, column};

pub fn show_modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    modal_content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(modal_content)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
        .into()
}

pub fn create_modal_view<'a, Message>(
    header_text: &'a str,
    message_text: &'a str,
    on_close: Message
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    container(
        column![
            text(header_text),

            text(message_text),

            button("OK")
                .on_press(on_close)
        ]
            .spacing(20)
            .padding(30)
    )
        .style(|theme| container::Style {
            border: iced::Border {
                color: Color::from_rgb(0.3, 1.0, 0.1),
                width: 1.0,
                radius: 10.into()
            },
            background: Some(Background::Color(Color::BLACK)),
            ..Default::default()
        })
        .into()
}