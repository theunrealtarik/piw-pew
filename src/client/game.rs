use crate::configs;
use crate::core::{AssetsHandle, NetUpdateHandle, RenderHandle, UpdateHandle, UserInterfaceHandle};
use crate::entities::{Enemy, GameWorldTile, Player, Projectile, Weapon};

use lib::packets::{ProjectileData, RawProjectileId};
use lib::types::SharedAssets;
use lib::utils::POINT_OFFSETS;
use lib::{
    packets::GameNetworkPacket,
    types::{RVector2, Tile},
};
use lib::{ENTITY_PROJECTILE_RADIUS, ENTITY_PROJECTILE_SPEED, WORLD_TILE_SIZE};

use raylib::{
    core::{text::Font, texture::Texture2D},
    prelude::*,
};
use renet::ClientId;
use renet::{
    transport::{ClientAuthentication, NetcodeClientTransport, NetcodeError},
    ConnectionConfig, DefaultChannel, RenetClient,
};

extern crate rmp_serde as rmps;
extern crate serde;
extern crate serde_derive;

use std::hash::Hash;
use std::sync::atomic::Ordering;
use std::time::Instant;
use std::{
    collections::HashMap,
    fs::File,
    io::{Error, ErrorKind, Read},
    net::{SocketAddr, UdpSocket},
    path::{Path, PathBuf},
    rc::Rc,
    time::Duration,
};
use strum::VariantArray;
use strum_macros::{Display, EnumIter, VariantArray};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Timers {
    WeaponShot(Duration),
    PlayerRepsawn,
}

pub struct Game {
    pub assets: SharedAssets<Assets>,
    pub player: Player,
    pub world: GameWorld,
    timers: GameTimer<Timers>,
}

impl Game {
    pub fn new(assets: SharedAssets<Assets>, settings: GameSettings) -> Self {
        Self {
            assets: Rc::clone(&assets),
            player: Player::new(settings.username, Rc::clone(&assets)),
            world: GameWorld::new(),
            timers: GameTimer::default(),
        }
    }
}

impl NetUpdateHandle for Game {
    fn net_update(&mut self, handle: &RaylibHandle, network: &mut GameNetwork) {
        let local_player = &mut self.player;
        let assets = self.assets.borrow();

        // reliable order messages
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
                                Tile::WALL_SIDE => TEXTURE::TILE_WALL_SIDE,
                                Tile::WALL_TOP => TEXTURE::TILE_WALL_TOP,
                                Tile::GROUND => TEXTURE::TILE_GROUND,
                            };
                            // hydration
                            if let Some(buffer) = assets.textures.get(&tile_texture) {
                                let buffer: &Texture2D = buffer;
                                let (w, h) = (buffer.width as f32, buffer.height as f32);

                                tiles.insert(
                                    (x, y),
                                    GameWorldTile::new(
                                        tile,
                                        tile_texture,
                                        x as u8,
                                        y as u8,
                                        w,
                                        h,
                                        WORLD_TILE_SIZE,
                                    ),
                                );
                            }
                        }

                        self.world.tiles = tiles;
                    }
                    GameNetworkPacket::NET_WORLD_PLAYERS(players) => {
                        self.world.enemies = players
                            .into_iter()
                            .map(|(id, data)| {
                                let client_id = ClientId::from_raw(id);
                                let mut enemy = Enemy::new(
                                    client_id,
                                    data.position.0,
                                    data.position.1,
                                    data.orientation,
                                    data.hp,
                                    Rc::clone(&self.assets),
                                );

                                enemy.inventory.select(data.weapon);
                                enemy.inventory.add(data.weapon, Weapon::new(data.weapon));

                                (client_id, enemy)
                            })
                            .collect::<Vec<(ClientId, Enemy)>>()
                            .into_iter()
                            .collect::<HashMap<ClientId, Enemy>>();
                    }
                    GameNetworkPacket::NET_PLAYER_JOINED(data) => {
                        let pos_x = data.position.0;
                        let pos_y = data.position.1;

                        if network.uuid == data._id {
                            local_player.name = data.name;
                            local_player.orientation = data.orientation;
                            local_player.rectangle.x = pos_x;
                            local_player.rectangle.y = pos_y;
                            local_player.ready = true;

                            local_player.inventory.select(data.weapon);
                            local_player
                                .inventory
                                .add(data.weapon, Weapon::new(data.weapon));
                        } else {
                            let id = ClientId::from_raw(data._id);
                            let mut enemy = Enemy::new(
                                id,
                                pos_x,
                                pos_y,
                                data.orientation,
                                data.hp,
                                Rc::clone(&self.assets),
                            );

                            enemy.inventory.select(data.weapon);
                            enemy.inventory.add(data.weapon, Weapon::new(data.weapon));
                            self.world.enemies.insert(id, enemy);
                        }
                    }
                    // these run at every frame
                    GameNetworkPacket::NET_PLAYER_WORLD_POSITION(id, (x, y)) => {
                        if let Some(enemy) = self.world.enemies.get_mut(&ClientId::from_raw(id)) {
                            enemy.rectangle.x = x;
                            enemy.rectangle.y = y;
                        }
                    }
                    GameNetworkPacket::NET_PROJECTILE_CREATE(projectile) => {
                        self.world.projectiles.insert(
                            projectile.id,
                            Projectile::new(
                                projectile.id,
                                projectile.position,
                                ENTITY_PROJECTILE_SPEED,
                                projectile.orientation,
                            ),
                        );
                    }
                    GameNetworkPacket::NET_PROJECTILE_IMPACT(id) => {
                        log::debug!("{:#?}", id);
                        self.world.projectiles.remove(&id);
                    }
                    _ => {}
                }
            };
        }

        while let Some(message) = network
            .client
            .receive_message(DefaultChannel::ReliableUnordered)
        {
            if let Ok(packet) = rmp_serde::from_slice::<GameNetworkPacket>(&message) {
                match packet {
                    GameNetworkPacket::NET_PLAYER_LEFT(id) => {
                        log::info!("player {:?} left", id);
                        self.world
                            .enemies
                            .remove(&ClientId::from_raw(id))
                            .expect("failed to remove player data");
                    }
                    _ => {}
                }
            }
        }

        while let Some(message) = network.client.receive_message(DefaultChannel::Unreliable) {
            if let Ok(packet) = rmp_serde::from_slice::<GameNetworkPacket>(&message) {
                match packet {
                    GameNetworkPacket::NET_PLAYER_ORIENTATION(id, orientation) => {
                        if let Some(puppet) = self.world.enemies.get_mut(&ClientId::from_raw(id)) {
                            puppet.orientation = orientation
                        }
                    }
                    _ => {}
                }
            }
        }

        // local player stuff
        local_player.net_update(handle, network);

        let position = local_player.on_move(handle);

        if self.world.in_bounds(
            position.x,
            position.y,
            local_player.rectangle.width,
            local_player.rectangle.height,
        ) {
            local_player.on_shoot(handle, |wpn, muzzle, theta| {
                if self
                    .timers
                    .after(Timers::WeaponShot(wpn.stats.fire_time), wpn.stats.fire_time)
                {
                    let p = Projectile::new(
                        u64::from_le_bytes(Uuid::new_v4().as_bytes()[..8].try_into().unwrap()),
                        (muzzle.x, muzzle.y),
                        ENTITY_PROJECTILE_SPEED,
                        theta,
                    );

                    network.client.send_message(
                        DefaultChannel::ReliableOrdered,
                        GameNetworkPacket::NET_PROJECTILE_CREATE(ProjectileData {
                            id: p.id,
                            position: (p.position.x, p.position.y),
                            grid: (p.grid.x, p.grid.y),
                            velocity: (p.velocity.x, p.velocity.y),
                            orientation: p.orientation,
                            shooter: network.transport.client_id(),
                            weapon: wpn.variant,
                        })
                        .serialized()
                        .unwrap(),
                    );
                }
            });

            let rectangle = Rectangle::new(
                position.x,
                position.y,
                local_player.rectangle.width,
                local_player.rectangle.height,
            );

            for tile in self
                .world
                .offset_tiles((local_player.grid.x, local_player.grid.y))
            {
                if let Some(tile) = tile {
                    if tile.variant != Tile::GROUND
                        && rectangle.check_collision_recs(&tile.dest_rect)
                    {
                        return;
                    }
                }
            }

            let position = local_player.move_to(position);
            network.client.send_message(
                DefaultChannel::ReliableUnordered,
                GameNetworkPacket::NET_PLAYER_WORLD_POSITION(
                    network.uuid,
                    (position.x, position.y),
                )
                .serialized()
                .unwrap(),
            );
        }
    }
}

impl RenderHandle for Game {
    fn render(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        let assets = self.assets.borrow();

        if !self.player.ready {
            return;
        }

        if self.world.tiles.len() > 0 {
            for tile in self.world.tiles.values() {
                let texture = assets.textures.get(&tile.texture).unwrap();

                d.draw_texture_pro(
                    texture,
                    tile.src_rect,
                    tile.dest_rect,
                    RVector2::zero(),
                    0.0,
                    Color::WHITE,
                );

                #[cfg(debug_assertions)]
                {
                    d.draw_text(
                        &format!("{:?} {:?}", tile.grid.x, tile.grid.y),
                        tile.dest_rect.x as i32,
                        tile.dest_rect.y as i32,
                        12,
                        Color::WHITE,
                    );
                    d.draw_rectangle_lines(
                        tile.dest_rect.x as i32,
                        tile.dest_rect.y as i32,
                        tile.dest_rect.width as i32,
                        tile.dest_rect.height as i32,
                        Color::new(0, 0, 255, 50),
                    );
                }
            }

            let (w, h) = (
                (self.world.tiles.len() as f32).sqrt() * WORLD_TILE_SIZE,
                (self.world.tiles.len() as f32).sqrt() * WORLD_TILE_SIZE,
            );
            d.draw_rectangle_lines_ex(Rectangle::new(0.0, 0.0, w, h), 1, Color::LIGHTGRAY);
        }

        self.player.render(d);

        for enemy in self.world.enemies.values_mut() {
            enemy.render(d);
        }

        self.world.render_projectiles(d);
    }
}

impl AssetsHandle for Game {
    type GameAssets = SharedAssets<Assets>;

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
        data: [u8; 256],
    ) -> Result<Self, NetcodeError> {
        let uuid = u64::from_le_bytes(Uuid::new_v4().as_bytes()[..8].try_into().unwrap());

        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let client = RenetClient::new(ConnectionConfig::default());

        let authentication = ClientAuthentication::Unsecure {
            server_addr,
            client_id: uuid,
            user_data: Some(data),
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

asset!(
    "ttf",
    FONT {
        FNT_POPPINS,
        FNT_JET
    }
);
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
        TILE_WALL_SIDE,
        TILE_WALL_TOP,
        TILE_GROUND,
        UI_LOADING,
        UI_LOGO,
    }
);

asset!("wav", SOUND {});

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
                let texture = *texture;

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
                let font = *font;

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

pub struct GameMenu {
    assets: SharedAssets<Assets>,
    rotation: f32,
}

impl GameMenu {
    pub fn new(assets: SharedAssets<Assets>) -> Self {
        Self {
            assets,
            rotation: 0.0,
        }
    }
}

impl RenderHandle for GameMenu {
    fn render(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        let assets = self.get_assets();
        let assets = assets.borrow();

        match (
            assets.textures.get(&TEXTURE::UI_LOGO),
            assets.textures.get(&TEXTURE::UI_LOADING),
        ) {
            (Some(logo_buf), Some(loading_buf)) => {
                let logo_texture: &Texture2D = logo_buf;
                let loading_texture: &Texture2D = loading_buf;

                let (logo_width, logo_height) =
                    (logo_texture.width as f32, logo_texture.height as f32);
                d.draw_texture_pro(
                    logo_texture,
                    Rectangle::new(0.0, 0.0, logo_width, logo_height),
                    Rectangle::new(
                        configs::window::WINDOW_CENTER_X,
                        configs::window::WINDOW_CENTER_Y,
                        logo_width / 4.0,
                        logo_height / 4.0,
                    ),
                    RVector2::new(logo_width / 8.0, logo_height / 4.0),
                    0.0,
                    Color::WHITE,
                );

                d.draw_texture_pro(
                    loading_texture,
                    Rectangle::new(
                        0.0,
                        0.0,
                        loading_texture.width as f32,
                        loading_texture.height as f32,
                    ),
                    Rectangle::new(
                        configs::window::WINDOW_CENTER_X,
                        configs::window::WINDOW_CENTER_Y + 75.0,
                        100.0,
                        100.0,
                    ),
                    RVector2::new(50.0, 50.0),
                    self.rotation,
                    Color::WHITE,
                );

                self.rotation -= 100.0 * d.get_frame_time();
            }
            _ => {}
        }
    }
}

impl AssetsHandle for GameMenu {
    type GameAssets = SharedAssets<Assets>;
    fn get_assets(&self) -> Self::GameAssets {
        Rc::clone(&self.assets)
    }
}

// game settings
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameSettings {
    pub username: String,
}

impl GameSettings {
    pub fn load(path: &PathBuf) -> Self {
        let default_user_settings = GameSettings {
            username: String::from("Player"),
        };

        match File::open(&path) {
            Ok(mut file) => {
                let mut buffer = String::new();
                if let Ok(bytes) = file.read_to_string(&mut buffer) {
                    if let Ok(settings) = serde_json::from_str::<Self>(&buffer) {
                        log::info!("read {} bytes from settings", bytes);

                        if !settings.username.is_empty() {
                            return settings;
                        }
                    }
                } else {
                    log::warn!("failed to read settings file");
                }
                default_user_settings
            }

            Err(ref err) if err.kind() == ErrorKind::NotFound => {
                log::error!("`settings.json` not found");
                log::warn!("creating a settings file...");

                if let Ok(mut file) = File::create(&path) {
                    if let Err(err) = serde_json::to_writer(&mut file, &default_user_settings) {
                        log::error!("failed to write default settings: {}", err);
                    }
                } else {
                    log::error!("failed to create `settings.json` file in {:?}", path);
                }

                return default_user_settings;
            }
            Err(err) => {
                log::error!("failed to open `settings.json` file: {}", err);
                default_user_settings
            }
        }
    }
}

pub struct GameWorld {
    tiles: HashMap<(i32, i32), GameWorldTile>,
    projectiles: HashMap<RawProjectileId, Projectile>,
    enemies: HashMap<ClientId, Enemy>,
}

impl GameWorld {
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            enemies: HashMap::new(),
            projectiles: HashMap::new(),
        }
    }

    fn offset_tiles(&self, (x, y): (i32, i32)) -> Vec<Option<&GameWorldTile>> {
        POINT_OFFSETS
            .into_iter()
            .map(|(dx, dy)| (x + dx as i32, y + dy as i32))
            .collect::<Vec<(i32, i32)>>()
            .into_iter()
            .map(|(gx, gy)| self.tiles.get(&(gx, gy)))
            .collect::<Vec<_>>()
    }

    fn render_projectiles(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        for (id, p) in &mut self.projectiles.clone() {
            if self.in_bounds(
                p.position.x,
                p.position.y,
                ENTITY_PROJECTILE_RADIUS,
                ENTITY_PROJECTILE_RADIUS,
            ) {
                let p = self.projectiles.get_mut(&id).unwrap();
                p.render(d);
            } else {
                self.projectiles.remove(id);
            }
        }
    }

    fn bounds(&self) -> (f32, f32) {
        let length = (self.tiles.len() as f32).sqrt() * WORLD_TILE_SIZE;
        (length, length)
    }

    fn in_bounds(&self, x: f32, y: f32, width: f32, height: f32) -> bool {
        let bounds = self.bounds();
        x > 0.0 && x <= bounds.0 - width && y > 0.0 && y < bounds.1 - height
    }
}

impl UserInterfaceHandle for Game {
    fn display(&mut self, d: &mut RaylibDrawHandle) {
        let local_player = &self.player;

        if let Some(wpn) = local_player.inventory.selected_weapon() {
            let ammo = format!(
                "{}/{}",
                wpn.stats.curr_mag_size,
                wpn.stats.curr_total_ammo - wpn.stats.curr_total_ammo
            );

            d.draw_text(&ammo, 10, 10, 24, Color::WHITE);
        }

        let is_alive = local_player.hp > 0;
        if is_alive {
            d.draw_rectangle_rounded(
                Rectangle::new(
                    configs::window::WINDOW_PADDING as f32,
                    configs::window::WINDOW_PADDING as f32,
                    100.0,
                    20.0,
                ),
                5.0,
                0,
                Color::new(0, 0, 0, 50),
            );
            d.draw_rectangle_rounded(
                Rectangle::new(
                    configs::window::WINDOW_PADDING as f32,
                    configs::window::WINDOW_PADDING as f32,
                    100.0,
                    20.0,
                ),
                5.0,
                0,
                Color::new(0, 0, 0, 50),
            );
        }
    }
}

struct GameTimer<T: Copy + PartialEq + Eq + std::hash::Hash> {
    value: HashMap<T, Timer<T>>,
}

impl<T: Copy + PartialEq + Eq + std::hash::Hash> Default for GameTimer<T> {
    fn default() -> Self {
        Self {
            value: HashMap::new(),
        }
    }
}

impl<T: Copy + Eq + std::hash::Hash> GameTimer<T> {
    fn after(&mut self, id: T, duration: Duration) -> bool {
        match self.value.get_mut(&id) {
            Some(timer) => {
                let now = Instant::now();
                let dt = now - timer.instant;

                if dt >= duration {
                    timer.instant = Instant::now();
                    true
                } else {
                    false
                }
            }
            None => {
                self.value.insert(id, Timer::new(id));
                true
            }
        }
    }
}

struct Timer<I> {
    instant: Instant,
    identifier: I,
}

impl<I> Timer<I> {
    fn new(identifier: I) -> Self {
        Self {
            instant: Instant::now(),
            identifier,
        }
    }
}
