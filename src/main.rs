use cursive::event::{Event, Key};
use cursive::views::{
    Button, Dialog, DummyView, EditView, LinearLayout, Panel, TextArea, TextView, ThemedView,
    ViewRef,
};
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

    siv.menubar()
        .add_subtree(
            "File",
            cursive::menu::Tree::new()
                .leaf("Reset", move |s| {
                    s.add_layer(
                        Dialog::new()
                            .title("Reset")
                            .content(TextView::new("TODO: Reset all the content"))
                            .dismiss_button("Close"),
                    )
                })
                .leaf("Save", move |_| {})
                .delimiter()
                .leaf("Quit", move |s| s.quit()),
        )
        .add_subtree(
            "Help",
            cursive::menu::Tree::new()
                .leaf("Help", move |_| {})
                .leaf("About rexer", move |s| {
                    s.add_layer(
                        Dialog::new()
                            .title("~rexer~")
                            .content(TextView::new("A simple regex tester"))
                            .dismiss_button("Ok"),
                    );
                }),
        );

    siv.set_autohide_menu(false);

    let output_box_theme = siv.current_theme().clone().with(|theme| {
        theme.palette[theme::PaletteColor::Primary] = theme::Color::Light(theme::BaseColor::White);
    });

    let tx = cursive::views::OnEventView::new(cursive::views::ThemedView::new(
        my_theme.clone(),
        TextArea::new().with_name("tsi").full_width().full_height(),
    ))
    .on_event(Event::Key(Key::Tab), |s| {
        let mut tt: ViewRef<TextArea> = s.find_name("tsi").unwrap();
        let mut prev_content = tt.get_content().to_owned();
        prev_content.push_str("    ");
        tt.set_content(prev_content.clone());
        tt.set_cursor(prev_content.len());
    });

    let output_box = Panel::new(ThemedView::new(
        output_box_theme,
        TextView::new("No result found! 🙈")
            .with_name("output_text")
            .full_height()
            .full_width(),
    ))
    .title("Result")
    .with_name("res_panel");

    let rx =
        cursive::views::ThemedView::new(my_theme, EditView::new().with_name("tri").full_width());

    siv.add_fullscreen_layer(Dialog::around(
        LinearLayout::vertical()
            .child(
                Panel::new(
                    LinearLayout::horizontal()
                        .child(rx)
                        .child(DummyView)
                        .child(Button::new("Execute", |_c| {
                            let mut ob: ViewRef<TextView> = _c.find_name("output_text").unwrap();
                            ob.set_content("Hello world")
                        }))
                        .full_width(),
                )
                .title("Regex input"),
            )
            .child(DummyView)
            .child(
                LinearLayout::horizontal()
                    .child(Panel::new(tx).title("Input Text").full_width())
                    .child(output_box),
            ),
    ));
    siv.run();
}
