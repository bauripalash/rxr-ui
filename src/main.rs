use cursive::event::{Event, Key};
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, TextArea};
use cursive::{theme, traits::*};

fn main() {
    let mut siv = cursive::default();

    //experimental theme;

    siv.load_theme_file("themes/dark.toml").unwrap();
    siv.clear_global_callbacks(Event::Key(Key::Tab));
    let my_theme = siv.current_theme().clone().with(|theme| {
        theme.palette[theme::PaletteColor::View] = theme::Color::Light(theme::BaseColor::White);
        theme.palette[theme::PaletteColor::Secondary] = theme::Color::parse("#3a3a3a").unwrap();
    });

    let tx = cursive::views::ThemedView::new(
        my_theme.clone(),
        TextArea::new().with_name("tsi").full_width().full_height(),
    );

    let rx =
        cursive::views::ThemedView::new(my_theme, EditView::new().with_name("tri").full_width());

    siv.add_fullscreen_layer(Dialog::around(
        LinearLayout::vertical()
            .child(
                LinearLayout::horizontal()
                    .child(rx)
                    .child(DummyView)
                    .child(Button::new("Execute", |_c| {}))
                    .full_width(),
            )
            .child(DummyView)
            .child(LinearLayout::horizontal().child(tx)),
    ));
    siv.run();
}
