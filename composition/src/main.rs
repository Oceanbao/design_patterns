/*
Files and Folders

File and Directory are both of trait Component with a single search method. For
a file, it will just look into the contents of the file; for a folder, it will go
through all files of that folder to find that keyword.
*/

mod fs {
    // mod.rs
    pub use file::File;
    pub use folder::Folder;

    pub trait Component {
        fn search(&self, keyword: &str);
    }

    mod file {
        use super::Component;

        pub struct File {
            name: &'static str,
        }

        impl File {
            pub fn new(name: &'static str) -> Self {
                Self { name }
            }
        }

        impl Component for File {
            fn search(&self, keyword: &str) {
                println!("Searching for keyword {} in file {}", keyword, self.name);
            }
        }
    }

    mod folder {
        use super::Component;

        pub struct Folder {
            name: &'static str,
            components: Vec<Box<dyn Component>>,
        }

        impl Folder {
            pub fn new(name: &'static str) -> Self {
                Self {
                    name,
                    components: vec![],
                }
            }

            pub fn add(&mut self, component: impl Component + 'static) {
                self.components.push(Box::new(component));
            }
        }

        impl Component for Folder {
            fn search(&self, keyword: &str) {
                println!(
                    "Searching recursively for keyword {} in folder {}",
                    keyword, self.name
                );

                for component in self.components.iter() {
                    component.search(keyword);
                }
            }
        }
    }
}

fn main() {
    use fs::{Component, File, Folder};

    let file1 = File::new("File 1");
    let file2 = File::new("File 2");
    let file3 = File::new("File 3");

    let mut folder1 = Folder::new("Folder 1");
    folder1.add(file1);

    let mut folder2 = Folder::new("Folder 2");
    folder2.add(file2);
    folder2.add(file3);
    folder2.add(folder1);

    folder2.search("rose");
}
