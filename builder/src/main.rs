// Builders
mod builders {
    // mod.rs
    // ------
    // mod car;
    // mod car_manuel;
    use crate::components::{CarType, Engine, GpsNavigator, Transmission};
    pub use car::CarBuilder;
    pub use car_manual::CarManualBuilder;

    pub trait Builder {
        type OutputType;
        fn set_car_type(&mut self, car_type: CarType);
        fn set_seats(&mut self, seats: u16);
        fn set_engine(&mut self, engine: Engine);
        fn set_transmission(&mut self, transmission: Transmission);
        fn set_gsp_navigator(&mut self, gps_navigator: GpsNavigator);
        fn build(self) -> Self::OutputType;
    }

    mod car {
        use crate::{
            cars::Car,
            components::{CarType, Engine, GpsNavigator, Transmission},
        };

        use super::Builder;

        pub const DEFAULT_FUEL: f64 = 5.0;

        #[derive(Default)]
        pub struct CarBuilder {
            car_type: Option<CarType>,
            engine: Option<Engine>,
            gps_navigator: Option<GpsNavigator>,
            seats: Option<u16>,
            transmission: Option<Transmission>,
        }

        impl Builder for CarBuilder {
            type OutputType = Car;

            fn set_car_type(&mut self, car_type: CarType) {
                self.car_type = Some(car_type);
            }

            fn set_engine(&mut self, engine: Engine) {
                self.engine = Some(engine);
            }

            fn set_gsp_navigator(&mut self, gps_navigator: GpsNavigator) {
                self.gps_navigator = Some(gps_navigator);
            }

            fn set_seats(&mut self, seats: u16) {
                self.seats = Some(seats);
            }

            fn set_transmission(&mut self, transmission: Transmission) {
                self.transmission = Some(transmission);
            }

            fn build(self) -> Car {
                Car::new(
                    self.car_type.expect("Please, set a car type"),
                    self.seats.expect("Please, set a number of seats"),
                    self.engine.expect("Please, set an engine configuration"),
                    self.transmission.expect("Please, set up transmission"),
                    self.gps_navigator,
                    DEFAULT_FUEL,
                )
            }
        }
    }

    mod car_manual {
        use crate::{
            cars::Manual,
            components::{CarType, Engine, GpsNavigator, Transmission},
        };

        use super::Builder;

        #[derive(Default)]
        pub struct CarManualBuilder {
            car_type: Option<CarType>,
            engine: Option<Engine>,
            gps_navigator: Option<GpsNavigator>,
            seats: Option<u16>,
            transmission: Option<Transmission>,
        }

        /// Builds a car manual instead of an actual car.
        impl Builder for CarManualBuilder {
            type OutputType = Manual;

            fn set_car_type(&mut self, car_type: CarType) {
                self.car_type = Some(car_type);
            }

            fn set_engine(&mut self, engine: Engine) {
                self.engine = Some(engine);
            }

            fn set_gsp_navigator(&mut self, gps_navigator: GpsNavigator) {
                self.gps_navigator = Some(gps_navigator);
            }

            fn set_seats(&mut self, seats: u16) {
                self.seats = Some(seats);
            }

            fn set_transmission(&mut self, transmission: Transmission) {
                self.transmission = Some(transmission);
            }

            fn build(self) -> Manual {
                Manual::new(
                    self.car_type.expect("Please, set a car type"),
                    self.seats.expect("Please, set a number of seats"),
                    self.engine.expect("Please, set an engine configuration"),
                    self.transmission.expect("Please, set up transmission"),
                    self.gps_navigator,
                )
            }
        }
    }
}

// Products
mod cars {
    // mod.rs
    // ------
    // mod car;
    // mod manuor
    pub use car::Car;
    pub use manual::Manual;

    mod car {
        use crate::components::{CarType, Engine, GpsNavigator, Transmission};

        pub struct Car {
            car_type: CarType,
            seats: u16,
            engine: Engine,
            transmission: Transmission,
            gps_navigator: Option<GpsNavigator>,
            fuel: f64,
        }

        impl Car {
            pub fn new(
                car_type: CarType,
                seats: u16,
                engine: Engine,
                transmission: Transmission,
                gps_navigator: Option<GpsNavigator>,
                fuel: f64,
            ) -> Self {
                Self {
                    car_type,
                    seats,
                    engine,
                    transmission,
                    gps_navigator,
                    fuel,
                }
            }

            pub fn car_type(&self) -> CarType {
                self.car_type
            }
            pub fn fuel(&self) -> f64 {
                self.fuel
            }

            pub fn set_fuel(&mut self, fuel: f64) {
                self.fuel = fuel;
            }

            pub fn seats(&self) -> u16 {
                self.seats
            }

            pub fn engine(&self) -> &Engine {
                &self.engine
            }

            pub fn transmission(&self) -> &Transmission {
                &self.transmission
            }

            pub fn gps_navigator(&self) -> &Option<GpsNavigator> {
                &self.gps_navigator
            }
        }
    }

    mod manual {
        use crate::components::{CarType, Engine, GpsNavigator, Transmission};

        pub struct Manual {
            car_type: CarType,
            seats: u16,
            engine: Engine,
            transmission: Transmission,
            gps_navigator: Option<GpsNavigator>,
        }

        impl Manual {
            pub fn new(
                car_type: CarType,
                seats: u16,
                engine: Engine,
                transmission: Transmission,
                gps_navigator: Option<GpsNavigator>,
            ) -> Self {
                Self {
                    car_type,
                    seats,
                    engine,
                    transmission,
                    gps_navigator,
                }
            }
        }

        impl std::fmt::Display for Manual {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(f, "Type of car: {:?}", self.car_type)?;
                writeln!(f, "Count of seats: {}", self.seats)?;
                writeln!(
                    f,
                    "Engine: volume - {}; mileage - {}",
                    self.engine.volume(),
                    self.engine.mileage()
                )?;
                writeln!(f, "Transmission: {:?}", self.transmission)?;
                match self.gps_navigator {
                    Some(_) => writeln!(f, "GPS Navigator: Functional")?,
                    None => writeln!(f, "GPS Navigator: N/A")?,
                };
                Ok(())
            }
        }
    }
}

mod components {
    #[derive(Copy, Clone, Debug)]
    pub enum CarType {
        CityCar,
        SportsCar,
        Suv,
    }

    #[derive(Debug)]
    pub enum Transmission {
        SingleSpeed,
        Manual,
        Automatic,
        SemiAutomatic,
    }

    pub struct Engine {
        volume: f64,
        mileage: f64,
        started: bool,
    }

    impl Engine {
        pub fn new(volume: f64, mileage: f64) -> Self {
            Self {
                volume,
                mileage,
                started: false,
            }
        }

        pub fn on(&mut self) {
            self.started = true;
        }

        pub fn off(&mut self) {
            self.started = false;
        }

        pub fn started(&self) -> bool {
            self.started
        }

        pub fn volume(&self) -> f64 {
            self.volume
        }

        pub fn mileage(&self) -> f64 {
            self.mileage
        }

        pub fn go(&mut self, mileage: f64) {
            if self.started() {
                self.mileage += mileage;
            } else {
                println!("Cannot go(), must start engine first!");
            }
        }
    }

    pub struct GpsNavigator {
        route: String,
    }

    impl GpsNavigator {
        pub fn new() -> Self {
            Self::from_route(
                "221b, Baker Street, London to Scotland Yard, 8-10 Broadway, London".into(),
            )
        }
        pub fn from_route(route: String) -> Self {
            Self { route }
        }
        pub fn route(&self) -> &String {
            &self.route
        }
    }
}

mod director {
    use crate::{
        builders::Builder,
        components::{CarType, Engine, GpsNavigator, Transmission},
    };
    /// Director knows how to build a car.
    ///
    /// However, a builder can build a car manual instead of an actual car,
    /// everything depends on the concrete builder.
    pub struct Director;

    impl Director {
        pub fn construct_sports_car(builder: &mut impl Builder) {
            builder.set_car_type(CarType::SportsCar);
            builder.set_seats(2);
            builder.set_engine(Engine::new(3.0, 0.0));
            builder.set_transmission(Transmission::SemiAutomatic);
            builder.set_gsp_navigator(GpsNavigator::new());
        }

        pub fn construct_city_car(builder: &mut impl Builder) {
            builder.set_car_type(CarType::CityCar);
            builder.set_seats(2);
            builder.set_engine(Engine::new(1.2, 0.0));
            builder.set_transmission(Transmission::Automatic);
            builder.set_gsp_navigator(GpsNavigator::new());
        }

        pub fn construct_suv(builder: &mut impl Builder) {
            builder.set_car_type(CarType::Suv);
            builder.set_seats(4);
            builder.set_engine(Engine::new(2.5, 0.0));
            builder.set_transmission(Transmission::Manual);
            builder.set_gsp_navigator(GpsNavigator::new());
        }
    }
}

fn main() {
    use crate::builders::{Builder, CarBuilder, CarManualBuilder};
    use cars::{Car, Manual};
    use director::Director;

    let mut car_builder = CarBuilder::default();

    // Director gets the concrete builder object from the client
    // (application code). That's because application knows better which
    // builder to use to get a specific product.
    Director::construct_sports_car(&mut car_builder);

    // The final product is often retrieved from a builder object, since
    // Director is not aware and not dependent on concrete builders and
    // products.
    let car: Car = car_builder.build();
    println!("Car built: {:?}\n", car.car_type());

    let mut manual_builder = CarManualBuilder::default();

    // Director may know several building recipes.
    Director::construct_city_car(&mut manual_builder);

    // The final car manual.
    let manual: Manual = manual_builder.build();
    println!("Car manual built:\n{}", manual);
}
