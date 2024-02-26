use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use raylib::prelude::*;
use strum::VariantArray;
use strum_macros::*;

macro_rules! asset {
  ($filetype:literal, $name:ident { $($variant:ident),* $(,)? }) => {
      #[allow(non_camel_case_types)]
      #[derive(Debug, Display, EnumIter, VariantArray, Copy, Clone, PartialEq, Eq, Hash)]
      pub enum $name {
          $($variant),*
      }

      impl $name {
          pub fn filename(&self) -> String {
             format!("{}.{}", self.to_string(), $filetype)
          }

          pub fn as_path(&self) -> PathBuf {
              std::env::current_dir().unwrap().join("assets").join(self.filename())
          }
      }
  };
}

/// fonts
asset!(
    "ttf",
    LFont {
        FNT_POPPINS,
        FNT_POPPINS_BLACK
    }
);

/// wapons
asset!(
    "png",
    LTexture {
        WPN_AKA,
        WPN_DEAN,
        WPN_PRRR,
        WPN_SHOTPEW,
        CONS_AMMO_BOX,
        CONS_OLIVE_OIL,
        TILE_WALL_SIDE,
        TILE_WALL_TOP,
        TILE_GROUND,
        UI_LOADING,
        UI_LOGO,
        UI_AKA_69,
        UI_DEAN_1911,
        UI_PRRR,
        UI_SHOTPEW,
        UI_LOCK,
    }
);

asset!("wav", LSound {});

#[derive(Debug)]
pub struct GameAssets {
    pub fonts: HashMap<LFont, Font>,
    pub textures: HashMap<LTexture, Texture2D>,
    pub sounds: HashMap<LSound, Sound>,
}

impl Default for GameAssets {
    fn default() -> Self {
        Self {
            fonts: HashMap::new(),
            textures: HashMap::new(),
            sounds: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct GameAssetsLoader {
    pub assets: GameAssets,
}

impl GameAssetsLoader {
    pub fn new(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &PathBuf,
    ) -> Result<Self, std::io::Error> {
        let mut assets = GameAssets::default();
        if Path::exists(path) {
            log::debug!("{:?}", path);

            for texture in LTexture::VARIANTS {
                let path = texture.as_path();
                let texture = *texture;

                if Path::exists(&path) {
                    log::debug!("{:?}", path);

                    match handle.load_texture(&thread, path.to_str().unwrap()) {
                        Ok(texture_buffer) => {
                            log::info!("texture loaded {:?}", path);
                            assets.textures.insert(texture, texture_buffer);
                        }
                        Err(err) => {
                            let error =
                                format!("failed to load texture {texture:#?} to video memory");
                            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, error));
                        }
                    }
                }
            }

            for font in LFont::VARIANTS {
                let path = font.as_path();
                let font = *font;

                if Path::exists(&path) {
                    log::debug!("{:?}", path);

                    match handle.load_font(&thread, path.to_str().unwrap()) {
                        Ok(font_buffer) => {
                            log::info!("font loaded {:?}", path);
                            assets.fonts.insert(font, font_buffer);
                        }
                        Err(err) => {
                            let error = format!("failed to load texture {font:#?} to video memory");
                            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, error));
                        }
                    }
                }
            }

            Ok(Self { assets })
        } else {
            let error = format!("couldn't locate {:?}", path);
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, error))
        }
    }
}
