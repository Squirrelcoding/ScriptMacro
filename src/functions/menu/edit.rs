use cursive::{
    traits::{Nameable, Resizable},
    views::{Dialog, EditView, SelectView},
    Cursive,
};

use super::super::lib;

pub fn edit(s: &mut Cursive) {
    let list = SelectView::<String>::new()
        .on_submit(foo)
        .with_name("macros");
    s.add_layer(
        Dialog::around(list)
            .title("Edit Macro")
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

fn foo(s: &mut Cursive, macro_name: &str) {
    let macro_nameb = macro_name.to_string();
    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(move |a: &mut Cursive, _: &str| {
                    bar(a, &macro_nameb);
                })
                .with_name("macro_name")
                .fixed_width(10),
        )
        .title("Macro Name")
        .button("Cancel", |s: &mut Cursive| {
            s.pop_layer();
            s.pop_layer();
        }),
    );
    s.call_on_name("macro_name", |a: &mut EditView| {
        a.set_content(macro_name);
    });
}

fn bar(s: &mut Cursive, macro_name: &str) {
    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(update_macro)
                .with_name("macro_path")
                .fixed_width(30),
        )
        .title("Macro Path"),
    );
    s.call_on_name("macro_path", |a: &mut EditView| {
        a.set_content(lib::get_macros().get(macro_name).unwrap());
    });
}

fn update_macro(s: &mut Cursive, _: &str) {
    let macro_name = s
        .call_on_name("macro_name", |a: &mut EditView| a.get_content())
        .unwrap();
    let macro_path = s
        .call_on_name("macro_path", |b: &mut EditView| b.get_content())
        .unwrap();
    let chosen_macro = s
        .call_on_name("macros", |b: &mut SelectView| {
            eprintln!("Selection: {}", b.selection().unwrap());
            b.selection().unwrap()
        })
        .unwrap();
    let mut macros = lib::get_macros();

    //If the new macro name doesn't exist in the hashmap
    if !macros.contains_key(macro_name.as_str()) {
        macros.remove(chosen_macro.as_str()); //Replace todo!() with og macro name
    }

    macros.insert(macro_name.to_string(), macro_path.to_string());
    lib::write_back(macros);
    s.add_layer(
        Dialog::text("Updated Macro")
            .button("OK", |a: &mut Cursive| {
                a.pop_layer();
                a.pop_layer();
                a.pop_layer();
                a.pop_layer();
            })
            .title("Message"),
    )
}
