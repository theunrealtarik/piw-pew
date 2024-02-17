#![allow(dead_code)]

use crate::configs::window;
use crate::entities::Player;

use lib::{
    core::{AssetsHandle, RenderHandle, UpdateHandle},
    types::RVector2,
};
use raylib::{
    core::{text::Font, texture::Texture2D},
    prelude::*,
};
use renet::{
    transport::{ClientAuthentication, NetcodeClientTransport, NetcodeError},
    ConnectionConfig, RenetClient,
};
use serde;
use std::{
    collections::HashMap,
    fs::File,
    io::{Error, ErrorKind, Read, Write},
    net::{SocketAddr, UdpSocket},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use strum::VariantArray;
use strum_macros::{Display, EnumIter, VariantArray};
use uuid::Uuid;

pub struct GameState {
    player: Player,
}

pub struct Game {
    pub local: GameState,
    pub assets: Arc<Assets>,
}

impl Game {
    pub fn new(assets: Arc<Assets>, settings: GameSettings) -> Self {
        Self {
            assets: Arc::clone(&assets),
            local: GameState {
                player: Player::new(settings.username, Arc::clone(&assets)),
            },
        }
    }
}

impl UpdateHandle for Game {
    fn update(&mut self, handle: &RaylibHandle) {
        self.local.player.update(handle);
    }
}

impl RenderHandle for Game {
    fn render(&mut self, d: &mut RaylibDrawHandle) {
        d.clear_background(window::WINDOW_BACKGROUND_COLOR);
        d.draw_fps(window::WINDOW_TOP_LEFT_X, window::WINDOW_TOP_LEFT_Y);
        self.local.player.render(d);
    }
}

impl AssetsHandle for Game {
    type GameAssets = Arc<Assets>;

    fn get_assets(&self) -> Self::GameAssets {
        Arc::clone(&self.assets)
    }
}

pub struct GameNetwork {
    pub client: RenetClient,
    pub transport: NetcodeClientTransport,
    pub current_time: Duration,
    pub authentication: ClientAuthentication,
    pub uuid: u64,
    pub protocol_id: u64,
}

impl GameNetwork {
    pub fn connect(
        server_addr: SocketAddr,
        current_time: Duration,
        protocol_id: u64,
    ) -> Result<Self, NetcodeError> {
        let uuid = u64::from_le_bytes(Uuid::new_v4().as_bytes()[..8].try_into().unwrap());

        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let client = RenetClient::new(ConnectionConfig::default());

        let authentication = ClientAuthentication::Unsecure {
            server_addr,
            client_id: uuid,
            user_data: None,
            protocol_id,
        };

        match NetcodeClientTransport::new(current_time, authentication.clone(), socket) {
            Ok(transport) => Ok(Self {
                client,
                transport,
                current_time,
                authentication,
                uuid,
                protocol_id,
            }),
            Err(err) => Err(err),
        }
    }
}

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

asset!("ttf", FONT { FNT_POPPINS });
asset!(
    "png",
    TEXTURE {
        WPN_AKA,
        WPN_DEAN,
        WPN_PRRR,
        WPN_SHOTPEW,
        PIK_AMMO_BOX,
        PIK_OLIVE_OIL,
        PIK_KEVLAR,
        UI_LOADING,
        UI_LOGO
    }
);

asset!(
    "wav",
    SOUND {
        SND_DEATH,
        SND_COLLECT,
        SND_WIN,
        SND_LOSE
    }
);

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
    pub assets: Assets,
}

impl GameAssets {
    pub fn load(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &PathBuf,
    ) -> Result<Self, Error> {
        let mut assets = Assets::new();
        if Path::exists(path) {
            log::debug!("{:?}", path);

            for texture in TEXTURE::VARIANTS {
                let path = texture.as_path();
                let texture = texture.clone();

                if Path::exists(&path) {
                    log::debug!("{:?}", path);

                    match handle.load_texture(&thread, path.to_str().unwrap()) {
                        Ok(texture_buffer) => {
                            log::info!("texture loaded {:?}", path);
                            assets.textures.insert(texture, texture_buffer);
                        }
                        Err(err) => {
                            log::error!("failed to load texture {:#?} to video memory", texture);
                            log::error!("{:?}", err);
                            std::process::exit(1);
                        }
                    }
                }
            }
            for font in FONT::VARIANTS {
                let path = font.as_path();
                let font = font.clone();

                if Path::exists(&path) {
                    log::debug!("{:?}", path);

                    match handle.load_font(&thread, path.to_str().unwrap()) {
                        Ok(font_buffer) => {
                            log::info!("font loaded {:?}", path);
                            assets.fonts.insert(font, font_buffer);
                        }
                        Err(err) => {
                            log::error!("failed to load texture {:#?} to video memory", font);
                            log::error!("{:?}", err);
                            std::process::exit(1);
                        }
                    }
                }
            }

            Ok(Self { assets: assets })
        } else {
            log::error!("couldn't locate {:?}", path);
            Err(Error::new(ErrorKind::NotFound, ""))
        }
    }
}

// game menu
pub struct GameMenu {
    assets: Arc<Assets>,
}

impl GameMenu {
    pub fn new(assets: Arc<Assets>) -> Self {
        Self { assets }
    }
}

impl RenderHandle for GameMenu {
    fn render(&mut self, d: &mut RaylibDrawHandle) {
        let assets = self.get_assets();

        match (
            assets.textures.get(&TEXTURE::UI_LOGO),
            assets.textures.get(&TEXTURE::UI_LOADING),
            assets.fonts.get(&FONT::FNT_POPPINS),
        ) {
            (Some(logo_buf), Some(loading_buf), Some(font_buf)) => {
                let logo_texture: &Texture2D = logo_buf;
                let loading_texture: &Texture2D = loading_buf;

                let scale = 0.2;
                let spacing = 5.0;

                let origin = |texture2d: &Texture2D, size: f32| RVector2 {
                    x: (texture2d.width as f32 * size) / 2.0,
                    y: (texture2d.height as f32 * size) / 2.0,
                };

                let center = |texture2d: &Texture2D, size: f32| RVector2 {
                    x: window::WINDOW_CENTER_X - origin(texture2d, size).x,
                    y: window::WINDOW_CENTER_Y - origin(texture2d, size).y,
                };

                d.draw_texture_ex(
                    logo_texture,
                    center(logo_texture, scale)
                        - RVector2 {
                            x: 0.0,
                            y: origin(logo_texture, scale).y - spacing,
                        },
                    0.0,
                    scale,
                    Color::WHITE,
                );

                d.draw_texture_ex(
                    loading_texture,
                    center(loading_texture, 0.2)
                        + RVector2 {
                            x: 0.0,
                            y: origin(logo_texture, scale).y + spacing,
                        },
                    0.0,
                    scale,
                    Color::WHITE,
                );

                let font: &Font = font_buf;
                let font_size = 25.0;
                d.draw_text_ex(
                    font,
                    "made with hate and agony with some sufference by txreq",
                    RVector2 {
                        x: window::WINDOW_BOTTOM_LEFT_X as f32 * font_size * 2.0,
                        y: window::WINDOW_BOTTOM_LEFT_Y as f32 * font_size * 2.0,
                    },
                    font_size,
                    1.0,
                    Color::WHITE,
                );
            }
            _ => {}
        }
    }
}

impl AssetsHandle for GameMenu {
    type GameAssets = Arc<Assets>;
    fn get_assets(&self) -> Self::GameAssets {
        Arc::clone(&self.assets)
    }
}

// game settings
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GameSettings {
    username: String,
}

impl GameSettings {
    pub fn load(path: &PathBuf) -> Self {
        let default_user_settings = GameSettings {
            username: String::from("Player"),
        };
        match File::open(&path) {
            Ok(ref mut file) => {
                let mut buffer = String::new();
                match file.read_to_string(&mut buffer) {
                    Ok(_bytes) => {
                        if let Ok(settings) = serde_json::from_str::<Self>(&buffer) {
                            return settings;
                        }

                        return default_user_settings;
                    }
                    Err(_) => default_user_settings,
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::NotFound => {
                    log::error!("`settings.json` not found");
                    log::warn!("creating a settings file...");

                    let mut file = File::create(&path).unwrap();
                    let buffer = serde_json::to_string(&default_user_settings).unwrap();
                    file.write_all(buffer.as_bytes()).unwrap();
                    default_user_settings
                }
                _ => {
                    log::error!("failed to create `settings.json` file in {:?}", path);
                    default_user_settings
                }
            },
        }
    }
}
