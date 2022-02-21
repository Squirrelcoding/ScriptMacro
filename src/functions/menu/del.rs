use cursive::{
    traits::Nameable,
    views::{Dialog, SelectView},
    Cursive,
};

use super::super::lib;

pub fn del(s: &mut Cursive) {
    let list = SelectView::<String>::new()
        .on_submit(foo)
        .with_name("macros");
    s.add_layer(
        Dialog::around(list)
            .title("Delete Macro")
            .button("Cancel", |s: &mut Cursive| {
                s.pop_layer();
            }),
    );
    for (k, _) in lib::get_macros() {
        s.call_on_name("macros", |a: &mut SelectView| {
            a.add_item_str(k);
        });
    }
}

fn foo(s: &mut Cursive, a: &str) {
    let mut hm = lib::get_macros();
    hm.remove(a);
    lib::write_back(hm);
    s.add_layer(
        Dialog::text(format!("Deleted Macro {}", a))
            .title("Deleted Macro")
            .button("Ok", |a: &mut Cursive| {
                a.pop_layer();
                a.pop_layer();
            }),
    );
}
