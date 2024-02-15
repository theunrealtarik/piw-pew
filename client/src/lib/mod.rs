pub mod types {
    extern crate nalgebra as na;

    pub type Color = raylib::color::Color;
}

pub mod core {
    use nalgebra::{Point2, Scale2, Vector2};
    use raylib::prelude::*;

    pub trait Update {
        fn update(&mut self, handle: &mut RaylibHandle);
    }

    pub trait Render {
        fn render(&mut self, draw_handle: &mut RaylibDrawHandle);
    }

    pub trait Entity {
        fn get_position(&self) -> &Point2<f32>;
        fn get_health(&self) -> &Health;
        fn get_scale(&self) -> &Scale2<f32>;
        fn get_velocity(&self) -> &Vector2<f32>;
    }

    pub struct Controller;
    impl Controller {
        pub fn new() -> Self {
            Self {}
        }

        pub fn on_press<F>(&self, handle: &RaylibHandle, key: KeyboardKey, f: F)
        where
            F: FnOnce(),
        {
            if handle.is_key_pressed(key) {
                f()
            }
        }

        pub fn on_release<F>(&self, handle: &RaylibHandle, key: KeyboardKey, f: F)
        where
            F: FnOnce(),
        {
            if handle.is_key_released(key) {
                f()
            }
        }

        pub fn on_hold<F>(&self, handle: &RaylibHandle, key: KeyboardKey, f: F)
        where
            F: FnOnce(),
        {
            if handle.is_key_down(key) {
                f()
            }
        }
    }

    pub struct Health {
        value: f32,
        threshold: f32,
        percentage: f32,
    }

    #[allow(dead_code)]
    impl Health {
        pub fn new(base: f32) -> Self {
            Self {
                value: base,
                threshold: base,
                percentage: 100.0,
            }
        }

        pub fn damage(&mut self, amount: f32) {
            self.value = self.value - amount;
            self.percentage = self.value / self.threshold * 100.0;
        }

        pub fn heal(&mut self, amout: f32) {
            self.value = self.value + amout;
            self.percentage = self.value / self.threshold * 100.0;
        }

        pub fn set(&mut self, hp: f32) {
            self.value = hp;
        }
    }
}

pub mod assets {
    use log;
    use std::{
        io::{Error, ErrorKind},
        path::{Path, PathBuf},
    };
    use strum::VariantArray;
    use strum_macros::{Display, EnumIter, VariantArray};

    macro_rules! asset {
        ($name:ident { $($variant:ident),* $(,)? }) => {
            #[allow(non_camel_case_types)]
            #[derive(Debug, Display, EnumIter, VariantArray)]
            pub enum $name {
                $($variant),*
            }

            impl $name {
                pub fn as_path(&self) -> PathBuf {
                    std::env::current_dir().unwrap().join("assets").join(self.to_string())
                }

                pub fn locate(assets_path: &PathBuf) {
                    for n in $name::VARIANTS {
                        let path = assets_path.join(n.to_string());
                        if Path::exists(&path) {
                            log::info!("{:?}", path);
                        } else {
                            log::error!("FILE DOESN'T EXIST {:?}", path);
                        }
                    }
                }
            }
        };
    }

    asset!(FONTS { FNT_POPPINS });
    asset!(TEXTURES {
        WPN_AKA,
        WPN_DEAN,
        WPN_PRRR,
        WPN_SHOTPEW,
        PIK_AMMO_BOX,
        PIK_OLIVE_OIL,
        PIK_KEVLAR,
    });
    asset!(SOUNDS {
        SND_DEATH,
        SND_COLLECT,
        SND_WIN,
        SND_LOSE
    });

    //
    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct Assets {}

    impl Assets {
        pub fn load(path: &PathBuf) -> Result<Self, Error> {
            if Path::exists(path) {
                log::info!("ASSETS FOLDER LOCATED {:?}", path);
                FONTS::locate(&path);
                TEXTURES::locate(&path);
                // SOUNDS::locate(&path);

                Ok(Self {})
            } else {
                log::error!("FAILED TO LOCATE ASSETS {:?}", path);
                Err(Error::new(ErrorKind::NotFound, ""))
            }
        }
    }
}
