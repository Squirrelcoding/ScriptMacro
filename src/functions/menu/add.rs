use std::rc::Rc;

use cursive::{
    traits::{Nameable, Resizable},
    views::{Dialog, EditView, HideableView},
    Cursive,
};

use super::super::lib;

pub fn add(s: &mut Cursive) {
    s.add_layer(
        HideableView::new(
            Dialog::around(
                EditView::new()
                    .on_submit(get_path)
                    .with_name("name")
                    .fixed_width(10),
            )
            .title("Enter macro name"),
        )
        .with_name("name_view"),
    )
}

fn get_path(s: &mut Cursive, _a: &str) {
    s.call_on_name("name_view", |a: &mut HideableView<Dialog>| {
        a.hide();
    });
    s.add_layer(
        HideableView::new(
            Dialog::around(
                EditView::new()
                    .on_submit(done)
                    .with_name("path")
                    .fixed_width(20),
            )
            .title("Enter macro script file path"),
        )
        .with_name("path_view"),
    );
}

fn done(s: &mut Cursive, macro_path: &str) {
    let mut macros = lib::get_macros();
    let macro_name = Rc::clone(
        &s.call_on_name("name", |a: &mut EditView| a.get_content())
            .unwrap(),
    )
    .to_string();
    macros.insert(macro_name, macro_path.to_string());
    lib::write_back(macros);
    s.add_layer(Dialog::text("Added Macro").button("Ok", |s: &mut Cursive| {
        s.pop_layer();
        s.pop_layer();
        s.pop_layer();
    }));
}
