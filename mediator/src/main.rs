/*
Rduce coupling between components via indirect communication through mediator

Top-Down Ownership approach suits Rust.
- mediator takes ownership of all components
- component doesn't preserve a reference to mediator; instead it gets
reference via a method call
- control flow starts from fn main() where mediator recevies external event/command
- mediator trait for interfaction between components is not the same as its external API
for receiving external events
*/
mod train_station {
    use std::collections::{HashMap, VecDeque};

    use crate::trains::Train;

    pub trait Mediator {
        fn notify_about_arrival(&mut self, train_name: &str) -> bool;
        fn notify_about_departure(&mut self, train_name: &str);
    }

    #[derive(Default)]
    pub struct TrainStation {
        trains: HashMap<String, Box<dyn Train>>,
        train_queue: VecDeque<String>,
        train_on_platform: Option<String>,
    }

    impl Mediator for TrainStation {
        fn notify_about_arrival(&mut self, train_name: &str) -> bool {
            if self.train_on_platform.is_some() {
                self.train_queue.push_back(train_name.into());
                false
            } else {
                self.train_on_platform.replace(train_name.into());
                true
            }
        }

        fn notify_about_departure(&mut self, train_name: &str) {
            if Some(train_name.into()) == self.train_on_platform {
                self.train_on_platform = None;

                if let Some(next_train_name) = self.train_queue.pop_front() {
                    let mut next_train = self.trains.remove(&next_train_name).unwrap();
                    next_train.arrive(self);
                    self.trains.insert(next_train_name.clone(), next_train);

                    self.train_on_platform = Some(next_train_name);
                }
            }
        }
    }

    impl TrainStation {
        pub fn accept(&mut self, mut train: impl Train + 'static) {
            if self.trains.contains_key(train.name()) {
                println!("{} has already arrived", train.name());
                return;
            }

            train.arrive(self);
            self.trains.insert(train.name().clone(), Box::new(train));
        }

        pub fn depart(&mut self, name: &'static str) {
            let train = self.trains.remove(name);
            if let Some(mut train) = train {
                train.depart(self);
            } else {
                println!("'{}' is not on the station!", name);
            }
        }
    }
}

mod trains {
    // mod.rs

    // mod freight_train;
    // mod passenger_train;

    pub use freight_train::FreightTrain;
    pub use passenger_train::PassengerTrain;

    use crate::train_station::Mediator;

    // A train gets a mediator object by reference.
    pub trait Train {
        fn name(&self) -> &String;
        fn arrive(&mut self, mediator: &mut dyn Mediator);
        fn depart(&mut self, mediator: &mut dyn Mediator);
    }

    mod freight_train {
        use super::Train;
        use crate::train_station::Mediator;

        pub struct FreightTrain {
            name: String,
        }

        impl FreightTrain {
            pub fn new(name: &'static str) -> Self {
                Self { name: name.into() }
            }
        }

        impl Train for FreightTrain {
            fn name(&self) -> &String {
                &self.name
            }

            fn arrive(&mut self, mediator: &mut dyn Mediator) {
                if !mediator.notify_about_arrival(&self.name) {
                    println!("Freight train {}: Arrival blocked, waiting", self.name);
                    return;
                }

                println!("Freight train {}: Arrived", self.name);
            }

            fn depart(&mut self, mediator: &mut dyn Mediator) {
                println!("Freight train {}: Leaving", self.name);
                mediator.notify_about_departure(&self.name);
            }
        }
    }

    mod passenger_train {
        use super::Train;
        use crate::train_station::Mediator;

        pub struct PassengerTrain {
            name: String,
        }

        impl PassengerTrain {
            pub fn new(name: &'static str) -> Self {
                Self { name: name.into() }
            }
        }

        impl Train for PassengerTrain {
            fn name(&self) -> &String {
                &self.name
            }

            fn arrive(&mut self, mediator: &mut dyn Mediator) {
                if !mediator.notify_about_arrival(&self.name) {
                    println!("Passenger train {}: Arrival blocked, waiting", self.name);
                    return;
                }

                println!("Passenger train {}: Arrived", self.name);
            }

            fn depart(&mut self, mediator: &mut dyn Mediator) {
                println!("Passenger train {}: Leaving", self.name);
                mediator.notify_about_departure(&self.name);
            }
        }
    }
}

fn main() {
    use train_station::TrainStation;
    use trains::{FreightTrain, PassengerTrain};

    let train1 = PassengerTrain::new("Train 1");
    let train2 = FreightTrain::new("Train 2");

    // Station has `accept` and depart methods,
    // but it also impl Mediator
    let mut station = TrainStation::default();

    // Station is taking ownership of the trains
    station.accept(train1);
    station.accept(train2);

    // train1/2 moved inside, but can use train names to
    // depart them
    station.depart("Train 1");
    station.depart("Train 2");
    station.depart("Train 3");
}
