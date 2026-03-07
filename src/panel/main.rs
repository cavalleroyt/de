use slint::prelude::*;

slint::include_modules!(); // will include the generated code for .slint files

fn main() {
    // decide where icon assets are located. If WINUX_ASSETS is set, use that;
    // otherwise fall back to a relative path that matches the repo layout.
    let asset_dir = std::env::var("WINUX_ASSETS").unwrap_or_else(|_| {
        let exe = std::env::current_exe().unwrap_or_default();
        let mut p = exe.clone();
        p.pop(); // strip filename
        p.join("../assets/icons").to_string_lossy().into_owned()
    });
    println!("using assets directory: {}", asset_dir);
    // create the taskbar window; in the future we may set it to a Wayland surface
    let taskbar = Taskbar::new();
    taskbar.set_icon_path(asset_dir);
    taskbar.set_width(800); // placeholder width; real compositor will resize
    taskbar.run();
}
