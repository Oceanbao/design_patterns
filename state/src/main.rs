/*
Allows object to change behaviour when its internal state changes.

Extracts state-related behaviours into separate state classes and forces
the original object to delegate the work to an instance of these classes, instead
of acting on its own.

State related to FSM, but instead of lots of conditional, each state is repr by a
type that impl a common state trait - transition depend on  particular trait implementation
for each state type.

Language feature - consuming self and Box<dyn State>
*/

mod players {
    pub struct Track {
        pub title: String,
        pub duration: u32,
        cursor: u32,
    }

    impl Track {
        pub fn new(title: &'static str, duration: u32) -> Self {
            Self {
                title: title.into(),
                duration,
                cursor: 0,
            }
        }
    }

    pub struct Player {
        playlist: Vec<Track>,
        current_track: usize,
        _volume: u8,
    }

    impl Default for Player {
        fn default() -> Self {
            Self {
                playlist: vec![
                    Track::new("Track 1", 180),
                    Track::new("Track 2", 165),
                    Track::new("Track 3", 197),
                    Track::new("Track 4", 205),
                ],
                current_track: 0,
                _volume: 25,
            }
        }
    }

    impl Player {
        pub fn next_track(&mut self) {
            self.current_track = (self.current_track + 1) % self.playlist.len();
        }

        pub fn prev_track(&mut self) {
            self.current_track =
                (self.playlist.len() + self.current_track - 1) % self.playlist.len();
        }

        pub fn play(&mut self) {
            self.track_mut().cursor = 10; // Playback imitation.
        }

        pub fn pause(&mut self) {
            self.track_mut().cursor = 43; // Paused at some moment.
        }

        pub fn rewind(&mut self) {
            self.track_mut().cursor = 0;
        }

        pub fn track(&self) -> &Track {
            &self.playlist[self.current_track]
        }

        fn track_mut(&mut self) -> &mut Track {
            &mut self.playlist[self.current_track]
        }
    }
}

mod state {
    use cursive::views::TextView;

    use crate::player::Player;

    pub struct StoppedState;
    pub struct PausedState;
    pub struct PlayingState;

    /// There is a base `State` trait with methods `play` and `stop` which make
    /// state transitions. There are also `next` and `prev` methods in a separate
    /// `impl dyn State` block below, those are default implementations
    /// that cannot be overridden.
    ///
    /// What is the `self: Box<Self>` notation? We use the state as follows:
    /// ```rust
    ///   let prev_state = Box::new(PlayingState);
    ///   let next_state = prev_state.play(&mut player);
    /// ```
    /// A method `play` receives a whole `Box<PlayingState>` object,
    /// and not just `PlayingState`. The previous state "disappears" in the method,
    /// in turn, it returns a new `Box<PausedState>` state object.
    pub trait State {
        fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State>;
        fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State>;
        fn render(&self, player: &Player, view: &mut TextView);
    }

    impl State for StoppedState {
        fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
            player.play();

            // Stopped -> Playing.
            Box::new(PlayingState)
        }

        fn stop(self: Box<Self>, _: &mut Player) -> Box<dyn State> {
            // Change no state.
            self
        }

        fn render(&self, _: &Player, view: &mut TextView) {
            view.set_content("[Stopped] Press 'Play'")
        }
    }

    impl State for PausedState {
        fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
            player.pause();

            // Paused -> Playing.
            Box::new(PlayingState)
        }

        fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
            player.pause();
            player.rewind();

            // Paused -> Stopped.
            Box::new(StoppedState)
        }

        fn render(&self, player: &Player, view: &mut TextView) {
            view.set_content(format!(
                "[Paused] {} - {} sec",
                player.track().title,
                player.track().duration
            ))
        }
    }

    impl State for PlayingState {
        fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
            player.pause();

            // Playing -> Paused.
            Box::new(PausedState)
        }

        fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
            player.pause();
            player.rewind();

            // Playing -> Stopped.
            Box::new(StoppedState)
        }

        fn render(&self, player: &Player, view: &mut TextView) {
            view.set_content(format!(
                "[Playing] {} - {} sec",
                player.track().title,
                player.track().duration
            ))
        }
    }

    // Default "next" and "prev" implementations for the trait.
    impl dyn State {
        pub fn next(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
            player.next_track();

            // Change no state.
            self
        }

        pub fn prev(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
            player.prev_track();

            // Change no state.
            self
        }
    }
}

fn execute(s: &mut Cursive, button: &'static str) {
    let PlayerApplication {
        mut player,
        mut state,
    } = s.take_user_data().unwrap();

    let mut view = s.find_name::<TextView>("Player Status").unwrap();

    // Here is how state mechanics work: the previous state
    // executes an action and returns a new state.
    // Each state has all 4 operations but reacts differently.
    state = match button {
        "Play" => state.play(&mut player),
        "Stop" => state.stop(&mut player),
        "Prev" => state.prev(&mut player),
        "Next" => state.next(&mut player),
        _ => unreachable!(),
    };

    state.render(&player, &mut view);

    s.set_user_data(PlayerApplication { player, state });
}

fn main() {
    use cursive::{
        event::Key,
        view::Nameable,
        views::{Dialog, TextView},
        Cursive,
    };
    use player::Player;
    use state::{State, StoppedState};

    // Application context: a music player and a state.
    struct PlayerApplication {
        player: Player,
        state: Box<dyn State>,
    }

    let mut app = cursive::default();

    app.set_user_data(PlayerApplication {
        player: Player::default(),
        state: Box::new(StoppedState),
    });

    app.add_layer(
        Dialog::around(TextView::new("Press Play").with_name("Player Status"))
            .title("Music Player")
            .button("Play", |s| execute(s, "Play"))
            .button("Stop", |s| execute(s, "Stop"))
            .button("Prev", |s| execute(s, "Prev"))
            .button("Next", |s| execute(s, "Next")),
    );

    app.add_global_callback(Key::Esc, |s| s.quit());

    app.run();
}
