#![allow(dead_code)]

use crate::configs::window;
use crate::entities::Player;

use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
    usize,
};

use lib::core::{Render, Update};
use lib::net::DELTA_TIME;
use raylib::{
    core::{text::Font, texture::Texture2D},
    prelude::*,
};
use renet::{transport::NetcodeClientTransport, RenetClient};

use strum::VariantArray;
use strum_macros::{Display, EnumIter, VariantArray};

pub struct GameState {
    player: Player,
}

pub struct Game {
    pub menu: Menu,
    pub local: GameState,
    pub handle: RaylibHandle,
    pub thread: RaylibThread,
    pub assets: Assets,
    pub network: GameNetwork,
}

impl Game {
    pub fn new(
        handle: RaylibHandle,
        thread: RaylibThread,
        assets: Assets,
        network: GameNetwork,
    ) -> Self {
        Self {
            menu: Menu {},
            local: GameState {
                player: Player::new(),
            },
            handle,
            thread,
            assets,
            network,
        }
    }

    pub fn update(&mut self) {
        if self.network.client.is_connected() {
            // revieve messages

            self.local.player.update(&mut self.handle);
        }
    }

    pub fn render(&mut self) {
        let mut d = self.handle.begin_drawing(&self.thread);
        d.clear_background(window::WINDOW_BACKGROUND_COLOR);

        if self.network.client.is_connecting() {
            self.menu.render(&mut d);
        } else if self.network.client.is_connected() {
            d.draw_fps(window::WINDOW_TOP_LEFT_X, window::WINDOW_TOP_LEFT_Y);

            // entites
            self.local.player.render(&mut d);
        }
    }

    pub fn run(&mut self) {
        let delta_time = DELTA_TIME;
        self.network.client.update(delta_time);
        self.network
            .transport
            .update(delta_time, &mut self.network.client)
            .unwrap();

        self.update();
        self.render();

        match self
            .network
            .transport
            .send_packets(&mut self.network.client)
        {
            Ok(_) => {}
            Err(err) => {
                log::error!("failed to send packets");
                log::error!("{:#?}", err);
            }
        };
        std::thread::sleep(delta_time);
    }
}

pub struct GameNetwork {
    transport: NetcodeClientTransport,
    client: RenetClient,
}

impl GameNetwork {
    pub fn new(transport: NetcodeClientTransport, client: RenetClient) -> Self {
        Self { transport, client }
    }
}

pub struct Menu;
impl Render for Menu {
    fn render(&mut self, d: &mut RaylibDrawHandle) {}
}

macro_rules! asset {
        ($name:ident { $($variant:ident),* $(,)? }) => {
            #[allow(non_camel_case_types)]
            #[derive(Debug, Display, EnumIter, VariantArray, Copy, Clone, PartialEq, Eq, Hash)]
            pub enum $name {
                $($variant),*
            }

            impl $name {
                pub fn as_path(&self) -> PathBuf {
                    std::env::current_dir().unwrap().join("assets").join(self.to_string())
                }
            }
        };
    }

asset!(FONT { FNT_POPPINS });
asset!(TEXTURE {
    WPN_AKA,
    WPN_DEAN,
    WPN_PRRR,
    WPN_SHOTPEW,
    PIK_AMMO_BOX,
    PIK_OLIVE_OIL,
    PIK_KEVLAR,
    UI_LOADING
});

asset!(SOUND {
    SND_DEATH,
    SND_COLLECT,
    SND_WIN,
    SND_LOSE
});

#[derive(Debug)]
pub struct Assets {
    pub fonts: HashMap<FONT, Font>,
    pub textures: HashMap<TEXTURE, Texture2D>,
}

impl Assets {
    fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            textures: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct GameAssets {
    pub handle: RaylibHandle,
    pub thread: RaylibThread,
    pub assets: Assets,
}

impl GameAssets {
    pub fn load(r: (RaylibHandle, RaylibThread), path: &PathBuf) -> Result<Self, Error> {
        let (handle, thread) = r;

        let assets = Assets::new();

        if Path::exists(path) {
            log::debug!("{:?}", path);

            // for texture in TEXTURE::VARIANTS {
            //     let path = texture.as_path();
            //     let texture = texture.clone();
            //
            //     if Path::exists(&path) {
            //         log::debug!("{:?}", path);
            //
            //         match handle.load_texture(&thread, path.to_str().unwrap()) {
            //             Ok(texture_buffer) => {
            //                 assets.textures.insert(texture, *texture_buffer);
            //             }
            //             Err(err) => {
            //                 log::error!("failed to load texture {:#?} to video memory", texture);
            //                 log::error!("{:?}", err);
            //                 std::process::exit(1);
            //             }
            //         }
            //     }
            // }
            // for font in FONT::VARIANTS {
            //     let path = font.as_path();
            //     let font = font.clone();
            //
            //     if Path::exists(&path) {
            //         log::debug!("{:?}", path);
            //
            //         match handle.load_font(&thread, path.to_str().unwrap()) {
            //             Ok(font_buffer) => {
            //                 assets.fonts.insert(font, *font_buffer);
            //             }
            //             Err(err) => {
            //                 log::error!("failed to load texture {:#?} to video memory", font);
            //                 log::error!("{:?}", err);
            //                 std::process::exit(1);
            //             }
            //         }
            //     }
            // }

            Ok(Self {
                handle,
                thread,
                assets,
            })
        } else {
            log::error!("couldn't locate {:?}", path);
            Err(Error::new(ErrorKind::NotFound, ""))
        }
    }
}
