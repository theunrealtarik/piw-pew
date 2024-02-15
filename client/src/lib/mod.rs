pub mod types {
    extern crate nalgebra as na;

    pub type Position = nalgebra::Vector2<f64>;
    pub type Color = graphics::types::Color;
    pub type Transform = graphics::types::Matrix2d;
}

pub mod assets {
    use log;
    use std::path::{Path, PathBuf};
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
                    std::env::current_dir().unwrap().join(self.to_string())
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
    pub struct Assets;
    impl Assets {
        pub fn load(path: &PathBuf) {
            if Path::exists(path) {
                log::info!("ASSETS FOLDER LOCATED {:?}", path);
                FONTS::locate(&path);
                TEXTURES::locate(&path);
                // SOUNDS::locate(&path);
            } else {
                log::error!("FAILED TO LOCATE ASSETS {:?}", path)
            }
        }
    }
}
