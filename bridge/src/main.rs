// Devices and Remotes:
// Device classes act as the implementation, whereas the Remotes act as the abstraction.

mod remotes {
    // mod.rs
    pub use advanced::AdvancedRemove;
    pub use basic::BasicRemote;

    use crate::device::Device;

    pub trait HashMutableDevice<D: Device> {
        fn device(&mut self) -> &mut D;
    }

    pub trait Remote<D: Device>: HashMutableDevice<D> {
        fn power(&mut self) {
            println!("Remote: power toggle");
            if self.device().is_enabled() {
                self.device().disable();
            } else {
                self.device().enable();
            }
        }

        fn volume_down(&mut self) {
            println!("Remote: volume down");
            let volume = self.device().volume();
            self.device().set_volume(volume - 10);
        }

        fn volume_up(&mut self) {
            println!("Remote: volume up");
            let volume = self.device().volume();
            self.device().set_volume(volume + 10);
        }

        fn channel_down(&mut self) {
            println!("Remote: channel down");
            let channel = self.device().channel();
            self.device().set_channel(channel - 1);
        }

        fn channel_up(&mut self) {
            println!("Remote: channel up");
            let channel = self.device().channel();
            self.device().set_channel(channel + 1);
        }
    }

    mod basic {
        use crate::device::Device;

        use super::{HashMutableDevice, Remote};

        pub struct BasicRemote<D: Device> {
            device: D,
        }

        impl<D: Device> BasicRemote<D> {
            pub fn new(device: D) -> Self {
                Self { device }
            }
        }

        impl<D: Device> HashMutableDevice<D> for BasicRemote<D> {
            fn device(&mut self) -> &mut D {
                &mut self.device
            }
        }

        impl<D: Device> Remote<D> for BasicRemote<D> {}
    }

    mod advanced {
        use crate::device::Device;

        use super::{HashMutableDevice, Remote};

        pub struct AdvancedRemove<D: Device> {
            device: D,
        }

        impl<D: Device> AdvancedRemove<D> {
            pub fn new(device: D) -> Self {
                Self { device }
            }

            pub fn mute(&mut self) {
                println!("Remote: mute");
                self.device.set_volume(0);
            }
        }

        impl<D: Device> HashMutableDevice<D> for AdvancedRemove<D> {
            fn device(&mut self) -> &mut D {
                &mut self.device
            }
        }

        impl<D: Device> Remote<D> for AdvancedRemove<D> {}
    }
}

mod device {
    // mod.rs
    pub use radio::Radio;
    pub use tv::Tv;

    pub trait Device {
        fn is_enabled(&self) -> bool;
        fn enable(&mut self);
        fn disable(&mut self);
        fn volume(&self) -> u8;
        fn set_volume(&mut self, percent: u8);
        fn channel(&self) -> u16;
        fn set_channel(&mut self, channel: u16);
        fn print_status(&self);
    }

    mod radio {
        use super::Device;

        #[derive(Clone)]
        pub struct Radio {
            on: bool,
            volume: u8,
            channel: u16,
        }

        impl Default for Radio {
            fn default() -> Self {
                Self {
                    on: false,
                    volume: 30,
                    channel: 1,
                }
            }
        }

        impl Device for Radio {
            fn is_enabled(&self) -> bool {
                self.on
            }

            fn enable(&mut self) {
                self.on = true;
            }

            fn disable(&mut self) {
                self.on = false;
            }

            fn volume(&self) -> u8 {
                self.volume
            }

            fn set_volume(&mut self, percent: u8) {
                self.volume = std::cmp::min(percent, 100);
            }

            fn channel(&self) -> u16 {
                self.channel
            }

            fn set_channel(&mut self, channel: u16) {
                self.channel = channel;
            }

            fn print_status(&self) {
                println!("------------------------------------");
                println!("| I'm radio.");
                println!("| I'm {}", if self.on { "enabled" } else { "disabled" });
                println!("| Current volume is {}%", self.volume);
                println!("| Current channel is {}", self.channel);
                println!("------------------------------------\n");
            }
        }
    }

    mod tv {
        use super::Device;

        #[derive(Clone)]
        pub struct Tv {
            on: bool,
            volume: u8,
            channel: u16,
        }

        impl Default for Tv {
            fn default() -> Self {
                Self {
                    on: false,
                    volume: 30,
                    channel: 1,
                }
            }
        }

        impl Device for Tv {
            fn is_enabled(&self) -> bool {
                self.on
            }

            fn enable(&mut self) {
                self.on = true;
            }

            fn disable(&mut self) {
                self.on = false;
            }

            fn volume(&self) -> u8 {
                self.volume
            }

            fn set_volume(&mut self, percent: u8) {
                self.volume = std::cmp::min(percent, 100);
            }

            fn channel(&self) -> u16 {
                self.channel
            }

            fn set_channel(&mut self, channel: u16) {
                self.channel = channel;
            }

            fn print_status(&self) {
                println!("------------------------------------");
                println!("| I'm TV set.");
                println!("| I'm {}", if self.on { "enabled" } else { "disabled" });
                println!("| Current volume is {}%", self.volume);
                println!("| Current channel is {}", self.channel);
                println!("------------------------------------\n");
            }
        }
    }
}

fn main() {
    use device::{Device, Radio, Tv};
    use remotes::{AdvancedRemove, BasicRemote, HashMutableDevice, Remote};

    fn test_device(device: impl Device + Clone) {
        println!("Tests with basic remote.");
        let mut basic_remote = BasicRemote::new(device.clone());
        basic_remote.power();
        basic_remote.device().print_status();

        println!("Tests with advanced remote.");
        let mut advanced_remote = AdvancedRemove::new(device);
        advanced_remote.power();
        advanced_remote.mute();
        advanced_remote.device().print_status();
    }

    test_device(Tv::default());
    test_device(Radio::default());
}
