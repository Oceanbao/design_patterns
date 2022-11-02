/*
trait SpecificTarget is incompatible with a call() which accepts trait Target only.
    fn call(target: impl Target);
Adapter helps to pass incompatible interface to call().
    let target = TargetAdapter::new(specific_target);
    call(target);
*/

mod adapter {
    use crate::{adaptee::SpecificTarget, target::Target};

    pub struct TargetAdapter {
        adaptee: SpecificTarget,
    }

    impl TargetAdapter {
        pub fn new(adaptee: SpecificTarget) -> Self {
            Self { adaptee }
        }
    }

    impl Target for TargetAdapter {
        fn request(&self) -> String {
            self.adaptee.specific_request().chars().rev().collect()
        }
    }
}

mod adaptee {
    pub struct SpecificTarget;

    impl SpecificTarget {
        pub fn specific_request(&self) -> String {
            ".tseuqer cificepS".into()
        }
    }
}

mod target {
    pub trait Target {
        fn request(&self) -> String;
    }

    pub struct OrdinaryTarget;

    impl Target for OrdinaryTarget {
        fn request(&self) -> String {
            "Ordinary request".into()
        }
    }
}

fn main() {
    use adaptee::SpecificTarget;
    use adapter::TargetAdapter;
    use target::{OrdinaryTarget, Target};

    fn call(target: impl Target) {
        println!("'{}'", target.request());
    }

    let target = OrdinaryTarget;

    print!("A compatible target can be directly called: ");
    call(target);

    let adaptee = SpecificTarget;

    println!(
        "Adaptee is incompatible with client: {}",
        adaptee.specific_request()
    );

    let adapter = TargetAdapter::new(adaptee);

    print!("But with adapter client can call its method: ");
    call(adapter);
}
