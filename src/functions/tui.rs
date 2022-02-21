use crate::functions::menu::{add, del, edit, view};
use cursive::{
    views::{Button, Dialog, LinearLayout},
    Cursive,
};

pub fn menu() {
    let mut siv = cursive::default();
    let buttons = LinearLayout::vertical()
        .child(Button::new("Add macro", add::add))
        .child(Button::new("View macros", view::view))
        .child(Button::new("Edit", edit::edit))
        .child(Button::new("Delete", del::del))
        .child(Button::new("Quit", Cursive::quit));
    siv.load_theme_file(format!(
        "{}/.mcrmng/theme.toml",
        std::env::var("HOME").unwrap()
    ))
    .unwrap();

    siv.add_layer(Dialog::around(Dialog::around(buttons)).title("Command Macro Manager"));

    siv.run();
}
