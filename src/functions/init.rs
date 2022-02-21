use std::{
    env,
    fs::{create_dir, write, File},
    io,
    path::Path,
};

pub fn init() {
    if Path::new(&format!("{}/.mcrmng", env::var("HOME").unwrap())).exists() {
        return;
    }

    let path = format!("{}/.mcrmng/macros.json", env::var("HOME").unwrap());
    create_dir(format!("{}/.mcrmng/", env::var("HOME").unwrap())).unwrap();
    File::create(&path).unwrap();
    write(&path, "{}".as_bytes()).unwrap();
    let mut resp = reqwest::blocking::get("https://raw.githubusercontent.com/Squirrelcoding/SoftsquirrelProps/main/projmngrstyle.toml").unwrap();
    let mut out =
        File::create(format!("{}/.mcrmng/theme.toml", env::var("HOME").unwrap())).unwrap();
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}
