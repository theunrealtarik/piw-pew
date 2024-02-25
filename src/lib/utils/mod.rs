use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

pub type SharedAssets<T> = Rc<RefCell<T>>;

pub fn center<T: Copy + std::ops::Div<Output = T> + From<u32>>(width: T, height: T) -> (T, T) {
    let half: T = T::from(2u32);
    let x = width / half;
    let y = height / half;
    (x, y)
}

pub fn raw_uuid() -> u64 {
    u64::from_le_bytes(Uuid::new_v4().as_bytes()[..8].try_into().unwrap())
}

pub static POINT_OFFSETS: [(i8, i8); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

/// env logger stuff
pub mod logging {
    use env_logger::{self, Env};

    pub struct Logger;
    impl Logger {
        pub fn env() -> Env<'static> {
            let env = Env::default()
                .filter_or("RUST_LOG", "server=trace,client=trace,lib=trace")
                .write_style_or("RUST_STYLE_LOG", "always");
            env
        }
    }
}

/// time stuff
pub mod time {
    use std::hash::Hash;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Timers {
        PlayerReloading,
        WeaponShot(Duration),
    }

    use std::collections::HashMap;
    pub use std::time::{Duration, Instant};

    pub struct Timer<T: Copy + PartialEq + Eq + Hash> {
        value: HashMap<T, Instant>,
    }

    impl<T: Copy + PartialEq + Eq + std::hash::Hash> Default for Timer<T> {
        fn default() -> Self {
            Self {
                value: HashMap::new(),
            }
        }
    }

    impl<T: Copy + Eq + Hash> Timer<T> {
        pub fn after(&mut self, id: T, duration: Duration) -> bool {
            match self.value.get_mut(&id) {
                Some(instant) => {
                    let now = Instant::now();
                    let dt = now - *instant;

                    if dt >= duration {
                        *instant = Instant::now();
                        true
                    } else {
                        false
                    }
                }
                None => {
                    self.add(id);
                    true
                }
            }
        }

        pub fn add(&mut self, id: T) {
            self.value.insert(id, Instant::now());
        }
    }
}
