use cursive::{
    traits::Nameable,
    views::{Dialog, LinearLayout, SelectView},
    Cursive,
};

use super::super::lib;

pub fn view(s: &mut Cursive) {
    let list = SelectView::<String>::new()
        .on_submit(foo)
        .with_name("macros");
    s.add_layer(
        Dialog::around(list)
            .title("Your Macros")
            .button("Cancel", |s: &mut Cursive| {
                s.pop_layer();
            }),
    );
    for (k, _) in lib::get_macros() {
        println!("Heere");
        s.call_on_name("macros", |a: &mut SelectView| {
            a.add_item_str(k);
        });
    }
}

fn foo(s: &mut Cursive, a: &str) {
    let hm = lib::get_macros();
    eprintln!("{:?}", hm);
    eprintln!("{:?}", hm.get("macro_name"));
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(Dialog::text(format!("Macro Name: {}", a)))
                .child(Dialog::text(format!(
                    "Macro Script Path: {}",
                    hm.get(a).unwrap()
                ))),
        )
        .title("Macro Info")
        .button("Ok", |a: &mut Cursive| {
            a.pop_layer();
            a.pop_layer();
        }),
    );
}
