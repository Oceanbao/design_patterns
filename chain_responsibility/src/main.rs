/*
Passing a request along a chain of potential handlers until one of them hanles it.

This pattern allows multiple objects to handle the request sans coupling sender class
to the concrete classes of the receivers; the chain can be composed dynamically at runtime
with any handler that follows a standard handler interface.

Demo: processing a patient through a chain of departments
    Patient -> Reception -> Doctor -> Medical -> Cashier
*/

// Request
mod patient {
    #[derive(Default)]
    pub struct Patient {
        pub name: String,
        pub registration_done: bool,
        pub doctor_check_up_done: bool,
        pub medicine_done: bool,
        pub payment_done: bool,
    }
}

// Handlers
mod department {
    // mod cashier;
    // mod doctor;
    // mod medical;
    // mod reception;
    pub use cashier::Cashier;
    pub use doctor::Doctor;
    pub use medical::Medical;
    pub use reception::Reception;

    use crate::patient::Patient;

    /// A single role of objects that make up a chain.
    /// A typical trait implementation must have `handle` and `next` methods,
    /// while `execute` is implemented by default and contains a proper chaining
    /// logic.
    pub trait Department {
        fn execute(&mut self, patient: &mut Patient) {
            self.handle(patient);

            if let Some(next) = &mut self.next() {
                next.execute(patient);
            }
        }

        fn handle(&mut self, patient: &mut Patient);
        fn next(&mut self) -> &mut Option<Box<dyn Department>>;
    }

    /// Helps to wrap an object into a boxed type.
    pub(self) fn into_next(
        department: impl Department + Sized + 'static,
    ) -> Option<Box<dyn Department>> {
        Some(Box::new(department))
    }

    mod cashier {
        use super::{Department, Patient};

        #[derive(Default)]
        pub struct Cashier {
            next: Option<Box<dyn Department>>,
        }

        impl Department for Cashier {
            fn handle(&mut self, patient: &mut Patient) {
                if patient.payment_done {
                    println!("Payment done");
                } else {
                    println!("Cashier getting money from a patient {}", patient.name);
                    patient.payment_done = true;
                }
            }

            fn next(&mut self) -> &mut Option<Box<dyn Department>> {
                &mut self.next
            }
        }
    }

    mod doctor {
        use super::{into_next, Department, Patient};

        pub struct Doctor {
            next: Option<Box<dyn Department>>,
        }

        impl Doctor {
            pub fn new(next: impl Department + 'static) -> Self {
                Self {
                    next: into_next(next),
                }
            }
        }

        impl Department for Doctor {
            fn handle(&mut self, patient: &mut Patient) {
                if patient.doctor_check_up_done {
                    println!("A doctor checkup is already done");
                } else {
                    println!("Doctor checking a patient {}", patient.name);
                    patient.doctor_check_up_done = true;
                }
            }

            fn next(&mut self) -> &mut Option<Box<dyn Department>> {
                &mut self.next
            }
        }
    }

    mod medical {
        use super::{into_next, Department, Patient};

        pub struct Medical {
            next: Option<Box<dyn Department>>,
        }

        impl Medical {
            pub fn new(next: impl Department + 'static) -> Self {
                Self {
                    next: into_next(next),
                }
            }
        }

        impl Department for Medical {
            fn handle(&mut self, patient: &mut Patient) {
                if patient.medicine_done {
                    println!("Medicine is already given to a patient");
                } else {
                    println!("Medical giving medicine to a patient {}", patient.name);
                    patient.medicine_done = true;
                }
            }

            fn next(&mut self) -> &mut Option<Box<dyn Department>> {
                &mut self.next
            }
        }
    }

    mod reception {
        use super::{into_next, Department, Patient};

        #[derive(Default)]
        pub struct Reception {
            next: Option<Box<dyn Department>>,
        }

        impl Reception {
            pub fn new(next: impl Department + 'static) -> Self {
                Self {
                    next: into_next(next),
                }
            }
        }

        impl Department for Reception {
            fn handle(&mut self, patient: &mut Patient) {
                if patient.registration_done {
                    println!("Patient registration is already done");
                } else {
                    println!("Reception registering a patient {}", patient.name);
                    patient.registration_done = true;
                }
            }

            fn next(&mut self) -> &mut Option<Box<dyn Department>> {
                &mut self.next
            }
        }
    }
}

fn main() {
    use department::{Cashier, Department, Doctor, Medical, Reception};
    use patient::Patient;

    let cashier = Cashier::default();
    let medical = Medical::new(cashier);
    let doctor = Doctor::new(medical);
    let mut reception = Reception::new(doctor);

    let mut patient = Patient {
        name: "John".into(),
        ..Patient::default()
    };

    // Reception handles a patient passing him to the next link in the chain.
    // Reception -> Doctor -> Medical -> Cashier.
    reception.execute(&mut patient);

    println!("\nThe patient has been already handled:\n");

    reception.execute(&mut patient);
}
