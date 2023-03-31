pub mod game {
    pub mod board;
    pub mod main;
}

pub mod menu {
    pub mod ui;
}

pub mod util {
    pub mod screen;
}

fn main() {
    crate::menu::ui::print_menu();
}
