use venom;
use venom::config::Config;

venom::make_config!(
    MOD_INFO,
    TestScriptConfig {
        toggled: bool = false,
        slider: f64 = 0.0,
        select: u32 = 0
    }
);

extern fn on_clicked() {
    venom::message_box!("Clicked", "Button Clicked", 0);
}

extern fn on_toggle_changed(val: bool) {
    let cfg = get_config();
    cfg.toggled = val;
    cfg.save();
}

extern fn on_select_changed(val: u32) {
    let cfg = get_config();
    cfg.select = val;
    cfg.save();
}

extern fn on_slider_changed(val: f64) {
    let cfg = get_config();
    cfg.slider = val;
    cfg.save();
}

fn create_menu() -> venom::menu::OptionsMenu {
    let cfg = get_config();
    let mut menu = venom::menu::OptionsMenu::new("Test Script");
    menu.add_header("Header".into());
    menu.add_button("Button".into(), Some("This is a button".into()), on_clicked);
    menu.add_toggle("Toggle".into(), Some("This is a toggle (glorified Select item)".into()), cfg.toggled, false, on_toggle_changed);
    menu.add_select("Select".into(), Some("This is a select".into()), vec!["Option 1".into(), "Option 2".into(), "Option 3".into()], cfg.select, 0, on_select_changed);
    menu.add_slider("Slider".into(), Some("This is a slider".into()), cfg.slider, 0.0, 1.0, 0.5, on_slider_changed);
    
    let mut submenu = venom::menu::OptionsMenu::new("Test Script::Submenu");
    submenu.add_button("Nothing to see here".into(), None, on_clicked);
    menu.add_submenu("Submenu".into(), Some("This is a submenu".into()), Box::new(submenu));

    menu
}

venom::init_mod!(
    "Test Script",
    "Example venom script.",
    "0.0.1",
    "L_",
    {
        venom::menu::register_menu(&MOD_INFO, create_menu);
    }
);