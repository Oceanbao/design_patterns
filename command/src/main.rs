/*
Converts requests or simple op into objects.

Allows deferred or remote execution of commands, storing command history, etc.

In Rust, a command interface should NOT hold a permanent reference to global, but
be passed from top to down as mutable

Demo: Text Editor
- Each button runs a separate command
- A command repr as object, can be pushed into history array for undoing
- TUI is created with cursive crate
*/

// Command Interface
mod command {
    // mod copy;
    // mod cut;
    // mod paste;

    pub use copy::CopyCommand;
    pub use cut::CutCommand;
    pub use paste::PasteCommand;

    /// Declares a method for executing (and undoing) a command.
    ///
    /// Each command receives an application context to access
    /// visual components (e.g. edit view) and a clipboard.
    pub trait Command {
        fn execute(&mut self, app: &mut cursive::Cursive) -> bool;
        fn undo(&mut self, app: &mut cursive::Cursive);
    }

    mod copy {
        use cursive::{views::EditView, Cursive};

        use super::Command;
        use crate::AppContext;

        #[derive(Default)]
        pub struct CopyCommand;

        impl Command for CopyCommand {
            fn execute(&mut self, app: &mut Cursive) -> bool {
                let editor = app.find_name::<EditView>("Editor").unwrap();
                let mut context = app.take_user_data::<AppContext>().unwrap();

                context.clipboard = editor.get_content().to_string();

                app.set_user_data(context);
                false
            }

            fn undo(&mut self, _: &mut Cursive) {}
        }
    }

    mod cut {
        use cursive::{views::EditView, Cursive};

        use super::Command;
        use crate::AppContext;

        #[derive(Default)]
        pub struct CutCommand {
            backup: String,
        }

        impl Command for CutCommand {
            fn execute(&mut self, app: &mut Cursive) -> bool {
                let mut editor = app.find_name::<EditView>("Editor").unwrap();

                app.with_user_data(|context: &mut AppContext| {
                    self.backup = editor.get_content().to_string();
                    context.clipboard = self.backup.clone();
                    editor.set_content("".to_string());
                });

                true
            }

            fn undo(&mut self, app: &mut Cursive) {
                let mut editor = app.find_name::<EditView>("Editor").unwrap();
                editor.set_content(&self.backup);
            }
        }
    }

    mod paste {
        use cursive::{views::EditView, Cursive};

        use super::Command;
        use crate::AppContext;

        #[derive(Default)]
        pub struct PasteCommand {
            backup: String,
        }

        impl Command for PasteCommand {
            fn execute(&mut self, app: &mut Cursive) -> bool {
                let mut editor = app.find_name::<EditView>("Editor").unwrap();

                app.with_user_data(|context: &mut AppContext| {
                    self.backup = editor.get_content().to_string();
                    editor.set_content(context.clipboard.clone());
                });

                true
            }

            fn undo(&mut self, app: &mut Cursive) {
                let mut editor = app.find_name::<EditView>("Editor").unwrap();
                editor.set_content(&self.backup);
            }
        }
    }
}

fn main() {
    mod command;

    use cursive::{
        traits::Nameable,
        views::{Dialog, EditView},
        Cursive,
    };

    use command::{Command, CopyCommand, CutCommand, PasteCommand};

    /// An application context to be passed into visual component callbacks.
    /// It contains a clipboard and a history of commands to be undone.
    #[derive(Default)]
    struct AppContext {
        clipboard: String,
        history: Vec<Box<dyn Command>>,
    }

    fn main() {
        let mut app = cursive::default();

        app.set_user_data(AppContext::default());
        app.add_layer(
            Dialog::around(EditView::default().with_name("Editor"))
                .title("Type and use buttons")
                .button("Copy", |s| execute(s, CopyCommand::default()))
                .button("Cut", |s| execute(s, CutCommand::default()))
                .button("Paste", |s| execute(s, PasteCommand::default()))
                .button("Undo", undo)
                .button("Quit", |s| s.quit()),
        );

        app.run();
    }

    /// Executes a command and then pushes it to a history array.
    fn execute(app: &mut Cursive, mut command: impl Command + 'static) {
        if command.execute(app) {
            app.with_user_data(|context: &mut AppContext| {
                context.history.push(Box::new(command));
            });
        }
    }

    /// Pops the last command and executes an undo action.
    fn undo(app: &mut Cursive) {
        let mut context = app.take_user_data::<AppContext>().unwrap();
        if let Some(mut command) = context.history.pop() {
            command.undo(app)
        }
        app.set_user_data(context);
    }
}
