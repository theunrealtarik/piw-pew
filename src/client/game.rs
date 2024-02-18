use crate::configs::{window, *};
use crate::core::{AssetsHandle, NetRenderHandle, NetUpdateHandle, RenderHandle, UpdateHandle};
use crate::entities::{GameWorldTile, Player};

use lib::{
    packets::GameNetworkPacket,
    types::{RVector2, Wall},
};
use raylib::{
    core::{text::Font, texture::Texture2D},
    prelude::*,
};
use renet::{
    transport::{ClientAuthentication, NetcodeClientTransport, NetcodeError},
    ConnectionConfig, DefaultChannel, RenetClient,
};

extern crate rmp_serde as rmps;
extern crate serde;
extern crate serde_derive;

use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{Error, ErrorKind, Read, Write},
    net::{SocketAddr, UdpSocket},
    path::{Path, PathBuf},
    rc::Rc,
    time::Duration,
};
use strum::VariantArray;
use strum_macros::{Display, EnumIter, VariantArray};
use uuid::Uuid;

pub struct Game {
    pub assets: Rc<RefCell<Assets>>,
    pub player: Player,
    pub world: GameWorld,
}

impl Game {
    pub fn new(assets: Rc<RefCell<Assets>>, settings: GameSettings) -> Self {
        Self {
            assets: Rc::clone(&assets),
            player: Player::new(settings.username, Rc::clone(&assets)),
            world: GameWorld::new(),
        }
    }
}

impl NetUpdateHandle for Game {
    type Network = GameNetwork;

    fn net_update(&mut self, handle: &RaylibHandle, network: &mut Self::Network) {
        let assets = self.assets.borrow();

        while let Some(message) = network
            .client
            .receive_message(DefaultChannel::ReliableOrdered)
        {
            if let Ok(packet) = rmp_serde::from_slice::<GameNetworkPacket>(&message) {
                match packet {
                    GameNetworkPacket::NET_WORLD_MAP(map) => {
                        let mut tiles = HashMap::new();
                        for ((x, y), tile) in map {
                            let tile_texture = match tile {
                                Wall::WALL_SIDE => TEXTURE::ENV_WALL_SIDE,
                                Wall::WALL_TOP => TEXTURE::ENV_WALL_TOP,
                            };
                            // hydration
                            if let Some(buffer) = assets.textures.get(&tile_texture) {
                                let buffer: &Texture2D = buffer;
                                let (w, h) = (buffer.width as f32, buffer.height as f32);
                                let scale = WORLD_TILE_SIZE / w;

                                tiles.insert((x, y), GameWorldTile::new(tile_texture, w, h, scale));
                            }
                        }

                        self.world.tiles = tiles;
                    }
                    GameNetworkPacket::NET_PLAYER_POSITION(_) => {}
                    GameNetworkPacket::NET_PLAYER_ORIENTATION_ANGLE(_) => {}
                    GameNetworkPacket::NET_PLAYER_NAME(_) => {}
                }
            };
        }

        self.player.update(handle);
    }
}

impl NetRenderHandle for Game {
    type Network = GameNetwork;
    fn net_render(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>, network: &mut Self::Network) {
        let _ = d.begin_mode2D(self.player.camera);

        let assets = self.assets.borrow();

        if self.world.tiles.len() > 0 {
            for ((x, y), tile) in &self.world.tiles {
                let texture = assets.textures.get(&tile.texture).unwrap();
                let position =
                    RVector2::new(*x as f32 * WORLD_TILE_SIZE, *y as f32 * WORLD_TILE_SIZE);
                d.draw_texture_pro(
                    texture,
                    tile.rectangle,
                    tile.rec_scale(position.x, position.y),
                    RVector2::new(0.0, 0.0),
                    0.0,
                    Color::WHITE,
                );
            }
        }

        self.player.render(d);
        d.draw_fps(window::WINDOW_TOP_LEFT_X, window::WINDOW_TOP_LEFT_Y);
    }
}

impl AssetsHandle for Game {
    type GameAssets = Rc<RefCell<Assets>>;

    fn get_assets(&self) -> Self::GameAssets {
        Rc::clone(&self.assets)
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
        ENV_WALL_SIDE,
        ENV_WALL_TOP,
        ENV_GROUND,
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

            Ok(Self { assets })
        } else {
            log::error!("couldn't locate {:?}", path);
            Err(Error::new(ErrorKind::NotFound, ""))
        }
    }
}

// game menu
pub struct GameMenu {
    assets: Rc<RefCell<Assets>>,
}

impl GameMenu {
    pub fn new(assets: Rc<RefCell<Assets>>) -> Self {
        Self { assets }
    }
}

impl RenderHandle for GameMenu {
    fn render(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        let assets = self.get_assets();
        let assets = assets.borrow();

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
    type GameAssets = Rc<RefCell<Assets>>;
    fn get_assets(&self) -> Self::GameAssets {
        Rc::clone(&self.assets)
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

// game world
pub struct GameWorld {
    tiles: HashMap<(usize, usize), GameWorldTile>,
}

impl GameWorld {
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }
}
