/*
GUI framework can organise its classes into independent libs:
- gui: interfaces for all components; no external depens
- windows-gui: implementation of base GUI -> gui
- macos-gui: implementation of Mac OS of base GUI -> gui

app: client using implementations of GUI;

Rust:
- generics (static dispatch)
- dynamic allocation (dynamic dispatch)
*/

// gui: Abstract Factory and Abstract products
// -------------------------------------------

// gui/lib.rs
mod gui {
    pub trait Button {
        fn press(&self);
    }
    pub trait CheckBox {
        fn switch(&self);
    }
    // Abstract Factory by generics
    pub trait GuiFactory {
        type B: Button;
        type C: CheckBox;

        fn create_button(&self) -> Self::B;
        fn create_checkbox(&self) -> Self::C;
    }
    // Abstract Factory by Tract Object
    pub trait GuiFactoryDynamic {
        fn create_button(&self) -> Box<dyn Button>;
        fn create_checkbox(&self) -> Box<dyn CheckBox>;
    }
}

// macos-gui: One family of products
// ---------------------------------

// macos-guid/lib.rs
// pub mod button;
// pub mod checkbox;
// pub mod factory;
mod macos_gui {

    mod button {
        use crate::gui::Button;

        pub struct MacOSButton {}

        impl MacOSButton {
            pub fn new() -> Self {
                Self {}
            }
        }

        impl Button for MacOSButton {
            fn press(&self) {
                println!("Press Button - MacOS");
            }
        }
    }

    mod checkbox {
        use crate::gui::CheckBox;

        pub struct MacOSCheckBox {}

        impl MacOSCheckBox {
            pub fn new() -> Self {
                Self {}
            }
        }

        impl CheckBox for MacOSCheckBox {
            fn switch(&self) {
                println!("Switch CheckBox - MacOS");
            }
        }
    }

    pub mod factory {
        use super::button::MacOSButton;
        use super::checkbox::MacOSCheckBox;
        use crate::gui::GuiFactory;
        use crate::gui::GuiFactoryDynamic;

        pub struct MacFactory {}

        impl MacFactory {
            pub fn new() -> Self {
                Self {}
            }
        }

        impl GuiFactory for MacFactory {
            type B = MacOSButton;
            type C = MacOSCheckBox;

            fn create_button(&self) -> Self::B {
                return MacOSButton::new();
            }
            fn create_checkbox(&self) -> Self::C {
                return MacOSCheckBox::new();
            }
        }

        impl GuiFactoryDynamic for MacFactory {
            fn create_button(&self) -> Box<dyn crate::gui::Button> {
                Box::new(MacOSButton {})
            }
            fn create_checkbox(&self) -> Box<dyn crate::gui::CheckBox> {
                Box::new(MacOSCheckBox {})
            }
        }
    }
}

// app: Client code with generic
// -----------------------------

mod app {
    // app/render.rs
    // -------------
    use super::gui::GuiFactory;
    use super::gui::{Button, CheckBox};

    pub fn render(factory: impl GuiFactory) {
        let button1 = factory.create_button();
        let button2 = factory.create_button();
        let checkbox1 = factory.create_checkbox();
        let checkbox2 = factory.create_checkbox();

        button1.press();
        button2.press();
        checkbox1.switch();
        checkbox2.switch();
    }

    // app/main.rs
    pub fn main() {
        use crate::macos_gui::factory::MacFactory;

        let macos = true;

        if macos {
            render(MacFactory::new());
        }
    }
}

// app_dyn: Client code with dynamic dispatch (trait object)
// ---------------------------------------------------------

mod app_dyn {

    use crate::gui::GuiFactoryDynamic;

    pub fn render(factory: &dyn GuiFactoryDynamic) {
        let button1 = factory.create_button();
        let button2 = factory.create_button();
        let checkbox1 = factory.create_checkbox();
        let checkbox2 = factory.create_checkbox();

        button1.press();
        button2.press();
        checkbox1.switch();
        checkbox2.switch();
    }

    use crate::macos_gui::factory::MacFactory;

    pub fn main() {
        let macos = true;

        // Allocate a factory object in runtime
        let factory: &dyn GuiFactoryDynamic = if macos {
            &MacFactory {}
        } else {
            &MacFactory {}
        };

        let button = factory.create_button();
        button.press();

        render(factory);
    }
}

fn main() {
    use crate::app::main as main_generic;
    use crate::app_dyn::main as main_dynamic;

    main_generic();
    main_dynamic();
}
