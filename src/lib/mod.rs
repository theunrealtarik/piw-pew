pub mod net {
    use std::time::Duration;

    pub const PROTOCOL_ID: u64 = 69;
    pub const DELTA_TIME: Duration = Duration::from_millis(12);
}

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

pub mod shared {}

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
        fn get_health(&self) -> &i8;
        fn get_scale(&self) -> &Scale2<f32>;
        fn get_velocity(&self) -> &Vector2<f32>;
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
