use crate::scaleform::value::{self, Value};

use self::item::*;

pub mod item;
mod external;

pub fn register_menu(mod_info: &crate::ModInfo, create: fn() -> OptionsMenu) {
    external::register_menu(mod_info.into(), create)
}

pub enum Callback {
    F64(extern fn(f64)),
    U32(extern fn(u32)),
    Bool(extern fn(bool))
}

impl Callback {
    pub fn call(&self, value: f64) {
        match self {
            Callback::F64(cb) => cb(value),
            Callback::U32(cb) => cb(value as u32),
            Callback::Bool(cb) => cb(value == 1.0),
        }
    }
}

#[repr(C)]
pub struct  OptionsMenu {
    header: String,
    // item_id: u32,
    items: Vec<Box<dyn OptionItem>>,
}

#[derive(Clone, Copy)]
pub enum MenuWidth {
    Wide,
    Narrow,
    // Width(u32)
}

impl OptionsMenu {
    pub fn new(header: &str) -> OptionsMenu {
        let header = format!("{header}\0");
        OptionsMenu {
            header,
            items: Vec::new(),
        }
    }

    pub fn header(&self) -> &str {
        self.header.as_str()
    }

    fn add_item(&mut self, item: Box<dyn OptionItem>) {
        self.items.push(item);
    }

    pub unsafe fn to_value(&self) -> Value {
        let mut menu_value = value::create_value! {
            "header": self.header.as_ptr(),
            "width": MenuWidth::Wide as u32
        };

        let mut menu_items = Value::create_array();
        for item in self.items.iter() {
            menu_items.push_back(&item.to_value());
        }

        menu_value.set_member("items", &menu_items);    
        menu_value
    }
}

impl OptionsMenu {
    pub fn add_blank(&mut self) {
        self.add_item(Box::new(Blank { } ));
    }

    pub fn add_header(&mut self, label: &'static str) {
        self.add_item(Box::new(Header { label }))
    }

    pub fn add_button(&mut self, label: String, desc: Option<String>, on_clickled: extern fn()) {
        let id = external::generate_id(&label);
        self.add_item(Box::new(Command::new(id, label, desc)));
        external::register_button_callback(id, on_clickled);
    }

    pub fn add_submenu(&mut self, label: String, desc: Option<String>, menu: Box<OptionsMenu>) {
        self.add_item(Box::new(Submenu::new(label, desc, menu)));
    }

    pub fn add_select(
        &mut self,
        label: String,
        desc: Option<String>,
        options: Vec<String>,
        selected: u32,
        default: u32,
        on_change: extern fn(u32)
    ) {
        let id = external::generate_id(&label);
        self.add_item(Box::new(Select::new(
            id, label, desc, options, selected, default,
        )));
        external::register_value_callback(id, Callback::U32(on_change));
    }

    pub fn add_toggle(&mut self, label: String, desc: Option<String>, value: bool, default: bool, on_change: extern fn(bool)) {
        let id = external::generate_id(&label);
        self.add_item(Box::new(Select::new(
            id, label, desc, vec!["Disabled".into(), "Enabled".into()], value as u32, default as u32,
        )));
        external::register_value_callback(id, Callback::Bool(on_change));
    }

    pub fn add_slider(
        &mut self,
        label: String,
        desc: Option<String>,
        value: f64,
        minimum: f64,
        maximum: f64,
        default: f64,
        on_change: extern fn(f64)
        // fidelity: u32,
    ) {
        let id = external::generate_id(&label);
        self.add_item(Box::new(Slider::new(
            id, label, desc, value, minimum, maximum, default, // fidelity,
        )));
        external::register_value_callback(id, Callback::F64(on_change));
    }

    // pub fn add_colour(&mut self, label: &'static str, desc: Option<&'static str>, colours: Vec<u32>, selected: u32, default: u32) {
    //     let id = external::generate_id(label);
    //     self.add_item(Box::new(Colour {
    //         id, label, desc, selected, default, colours: colours.iter().enumerate().map(|(i, col)| {
    //             ColourElement {
    //                 colour_index: i as u32,
    //                 colour_type: ColourType::Normal,
    //                 colour_value: *col,
    //                 colour_name: "TODO\0"
    //             }
    //         }).collect()  
    //     }))
    // }

    // pub fn add_keybind(&mut self, label: &'static str, desc: Option<&'static str>, binding: &'static str, is_icon_type: bool, locked: bool, can_reset: bool) {
    //     let id = self.item_id();
    //     self.add_item(Box::new(Keybind {
    //         id, label, desc, binding, is_icon_type, locked, can_reset
    //     }))
    // }
}