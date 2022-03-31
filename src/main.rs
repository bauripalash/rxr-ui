//use cursive::backends::termion::termion::color::Color;
use cursive::event::{Event, Key};
use cursive::theme::{BaseColor, Color, Effect, Style};
use cursive::utils::markup::StyledString;
use cursive::views::{
    Button, Dialog, DummyView, EditView, LinearLayout, Panel, TextArea, TextView, ThemedView,
    ViewRef, ScrollView,
};
use cursive::{theme, traits::*};
use regex::Regex;

// Issue => Screen flickers when using crossterm backend with `cursive_tabs`

fn main() {
    let mut siv = cursive::default();
    siv.set_window_title("~rexer~");
    //siv.set_fps(30);
    //siv.set_autorefresh(true);
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
                            .button("Reset", |s| {
                                let mut regex_input: ViewRef<EditView> =
                                    s.find_name("tri").unwrap();
                                regex_input.set_content("");
                                let mut string_input: ViewRef<TextArea> =
                                    s.find_name("tsi").unwrap();
                                string_input.set_content("");
                                let mut output_box: ViewRef<TextView> =
                                    s.find_name("output_text").unwrap();
                                output_box.set_content("");
                                s.pop_layer().unwrap();
                            })
                            .dismiss_button("Cancel")
                            .with_name("reset_dialog"),
                    )
                })
                .delimiter()
                .leaf("Quit", move |s| s.quit()),
        )
        .add_subtree(
            "Help",
            cursive::menu::Tree::new()
                .leaf("Help", move |s| {
                    s.add_layer(
                        Dialog::new()
                            .title("Help")
                            .content(TextView::new("TODO: Show Help"))
                            .dismiss_button("Close"),
                    )
                })
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
            .full_height(),
    ))
    .title("Result")
    .with_name("res_panel");

    let rx =
        cursive::views::ThemedView::new(my_theme, EditView::new().with_name("tri").full_width());

    //siv.add_fullscreen_layer(
    let main_dialog = Dialog::around(
        LinearLayout::vertical()
            .child(DummyView)
            .child(
                Panel::new(
                    LinearLayout::horizontal()
                        .child(rx)
                        .child(DummyView)
                        .child(Button::new("Execute", |_c| {
                            let mut ob: ViewRef<TextView> = _c.find_name("output_text").unwrap();
                            let edv: ViewRef<EditView> = _c.find_name("tri").unwrap();
                            let raw_reg = Regex::new(&edv.get_content().to_string());
                            let input_box: ViewRef<TextArea> = _c.find_name("tsi").unwrap();
                            let input_string = input_box.get_content().to_owned();
                            match raw_reg {
                                Ok(r) => {
                                    let mut tmp: Vec<String> = vec![];
                                    let mut styledtext = StyledString::plain("");
                                    for (i,f) in r.find_iter(&input_string).enumerate() {
                                        let tmp_output = format!("[M{}] String->{} | Starts at->{} | Ends at->{} | Range->{:?}" , &i+1 ,&f.as_str() , &f.start() , &f.end(), &f.range() );

                                        tmp.push(tmp_output);

                                        styledtext.append(StyledString::styled(format!("M[{}] " , i) , Style::from(Color::Light(BaseColor::White)).combine(Effect::Bold)));
                                        styledtext.append(StyledString::styled(&f.as_str().to_owned(), Color::Light(BaseColor::Green)));
                                        styledtext.append_plain(" : ");
                                        styledtext.append(StyledString::styled(format!("<{}...{}>" , &f.start() , &f.end()), Style::from(Color::Light(BaseColor::Cyan)).combine(Effect::Underline)));
                                        styledtext.append_plain("\n"); 
                                    }

                                    //ob.set_content(tmp.join("\n"));
                                    ob.set_content(styledtext);
                                }
                                Err(_) => {
                                    ob.set_content("Invalid Regex");
                                }
                            }
                            //ob.set_content(edv.get_content().to_string())
                        }))
                        .full_width(),
                )
                .title("Regex input"),
            )
            .child(DummyView)
            .child(
                LinearLayout::horizontal()
                    .child(Panel::new(tx).title("Input Text").full_width())
                    .child(ScrollView::new(output_box)),
            ),
    )
    .with_name("main");
    //);
    //let main_panel = TabPanel::new()
    //    .with_tab(main_dialog)
    //    .with_name("experiment");
    siv.add_fullscreen_layer(main_dialog);
    siv.run();
}
