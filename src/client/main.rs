use env_logger;
use raylib::prelude::*;
use strum::VariantArray;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::{cell::RefCell, net::SocketAddr, rc::Rc, time::SystemTime};

use lib::prelude::*;
use lib::types::*;
use lib::utils;

fn main() {
    env_logger::init_from_env(Logger::env());

    let current_dir = std::env::current_dir().unwrap();

    let server_addr: SocketAddr = "127.0.0.1:6969".parse().expect("failed to server socket");
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let (mut handle, thread) = raylib::init()
        .title(WINDOW_NAME)
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .build();

    let ga_loaded = match GameAssetsLoader::new(&mut handle, &thread, &current_dir.join("assets")) {
        Ok(assets) => assets,
        Err(_) => {
            log::error!("failed to load assets");
            std::process::exit(1);
        }
    };
    let assets = Rc::new(RefCell::new(ga_loaded.assets));

    let settings = GameSettings::load(&current_dir.join("settings.json"));
    let mut data: [u8; 256] = [0; 256];
    for (index, byte) in settings.username.bytes().enumerate() {
        if index >= INITIAL_PAYLOAD_SIZE {
            break;
        }

        data[index] = byte;
    }

    let mut network = match GameNetwork::connect(server_addr, current_time, PROTOCOL_ID, data) {
        Ok(net) => {
            log::info!("network layer is set");
            net
        }
        Err(_) => {
            log::error!("failed to setup network layer");
            std::process::exit(1);
        }
    };

    let mut menu = GameMenu::new(Rc::clone(&assets));
    let mut game = Game::new(assets.clone(), settings);

    while !handle.window_should_close() {
        let delta_time = DELTA_TIME;

        network.client.update(delta_time);
        network
            .transport
            .update(delta_time, &mut network.client)
            .unwrap();

        if network.client.is_connected() {
            game.update(&handle);
            game.net_update(&handle, &mut network);
        }

        let mut draw = handle.begin_drawing(&thread);
        draw.clear_background(WINDOW_BACKGROUND_COLOR);

        let mut draw_2d = draw.begin_mode2D(game.player.camera);

        if network.client.is_connecting() {
            menu.render(&mut draw_2d);
        } else if network.client.is_connected() {
            game.render(&mut draw_2d);
            std::mem::drop(draw_2d);
            game.display(&mut draw);
        }

        match network.transport.send_packets(&mut network.client) {
            Ok(_) => {}
            Err(err) => {
                log::error!("failed to send packets");
                log::error!("{:#?}", err);
            }
        };
        std::thread::sleep(delta_time);
    }
}

struct Game {
    pub assets: SharedAssets<GameAssets>,
    pub player: Player,
    pub world: GameWorld,
}

impl Game {
    pub fn new(assets: SharedAssets<GameAssets>, settings: GameSettings) -> Self {
        Self {
            assets: Rc::clone(&assets),
            player: Player::new(settings.username, Rc::clone(&assets)),
            world: GameWorld::new(),
        }
    }
}

impl UpdateHandle for Game {
    fn update(&mut self, handle: &RaylibHandle) {
        self.player.update(handle);
    }
}

impl NetUpdateHandle for Game {
    type Network = GameNetwork;

    fn net_update(&mut self, handle: &RaylibHandle, network: &mut Self::Network) {
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
                                TileVariant::WALL_SIDE => LTexture::TILE_WALL_SIDE,
                                TileVariant::WALL_TOP => LTexture::TILE_WALL_TOP,
                                TileVariant::GROUND => LTexture::TILE_GROUND,
                            };
                            // hydration
                            if let Some(buffer) = assets.textures.get(&tile_texture) {
                                let buffer: &Texture2D = buffer;
                                let (w, h) = (buffer.width as f32, buffer.height as f32);

                                tiles.insert(
                                    (x, y),
                                    Tile::new(
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
                                    data.health,
                                    Rc::clone(&self.assets),
                                );

                                enemy.inventory.select(data.weapon);
                                enemy.inventory.add(Weapon::new(data.weapon));

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
                            local_player.orientation = data.orientation;
                            local_player.rectangle.x = pos_x;
                            local_player.rectangle.y = pos_y;
                            local_player.inventory.cash = data.cash;
                            local_player.ready = true;

                            local_player.inventory.select(data.weapon);
                            local_player.inventory.add(Weapon::new(data.weapon));
                        } else {
                            let id = ClientId::from_raw(data._id);
                            let mut enemy = Enemy::new(
                                id,
                                pos_x,
                                pos_y,
                                data.orientation,
                                data.health,
                                Rc::clone(&self.assets),
                            );

                            enemy.inventory.select(data.weapon);
                            enemy.inventory.add(Weapon::new(data.weapon));
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
                    GameNetworkPacket::NET_PROJECTILE_IMPACT(pid, cid, damage) => {
                        self.world.projectiles.remove(&pid);

                        if let Some(cid) = cid {
                            if cid == network.transport.client_id().raw() {
                                local_player.health -= damage as Health;
                                local_player.health = nalgebra::clamp(
                                    local_player.health,
                                    0,
                                    ENTITY_PLAYER_MAX_HEALTH,
                                );
                            } else if let Some(puppet) =
                                self.world.enemies.get_mut(&ClientId::from_raw(cid))
                            {
                                puppet.health -= damage as Health;
                                puppet.health =
                                    nalgebra::clamp(puppet.health, 0, ENTITY_PLAYER_MAX_HEALTH);
                            }
                        }
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
                    GameNetworkPacket::NET_PLAYER_RESPAWN(d_id, data) => {
                        if d_id == network.transport.client_id().raw() {
                            local_player.rectangle.x = data.position.0;
                            local_player.rectangle.y = data.position.1;
                            local_player.health = data.health;
                            local_player.ready = true;
                            local_player.inventory.cash = data.cash;
                            local_player.inventory.reset_weapons();
                        }
                    }
                    GameNetworkPacket::NET_PLAYER_KILL_REWARD(data) => {
                        local_player.inventory.cash = data.cash;
                    }
                    GameNetworkPacket::NET_PLAYER_WEAPON(variant) => {
                        local_player.inventory.add(variant.weapon_instance());
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
                let p = Projectile::new(
                    utils::raw_uuid(),
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
                        shooter: network.transport.client_id().raw(),
                        damage: *wpn.stats.damage(),
                    })
                    .serialized()
                    .unwrap(),
                );
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
                    if tile.variant != TileVariant::GROUND
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
                        Color::new(255, 255, 255, 50),
                    );
                    d.draw_rectangle_lines(
                        tile.dest_rect.x as i32,
                        tile.dest_rect.y as i32,
                        tile.dest_rect.width as i32,
                        tile.dest_rect.height as i32,
                        Color::new(255, 255, 255, 50),
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
    type GameAssets = SharedAssets<GameAssets>;

    fn get_assets(&self) -> Self::GameAssets {
        Rc::clone(&self.assets)
    }
}

pub struct GameMenu {
    assets: SharedAssets<GameAssets>,
    rotation: f32,
}

impl GameMenu {
    pub fn new(assets: SharedAssets<GameAssets>) -> Self {
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
            assets.textures.get(&LTexture::UI_LOGO),
            assets.textures.get(&LTexture::UI_LOADING),
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
                        WINDOW_CENTER_Y,
                        WINDOW_CENTER_X,
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
                    Rectangle::new(WINDOW_CENTER_X, WINDOW_CENTER_Y + 75.0, 100.0, 100.0),
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
    type GameAssets = SharedAssets<GameAssets>;
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

            Err(ref err) if err.kind() == std::io::ErrorKind::NotFound => {
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
    tiles: HashMap<(i32, i32), Tile>,
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

    fn offset_tiles(&self, (x, y): (i32, i32)) -> Vec<Option<&Tile>> {
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

        let assets = self.assets.borrow();
        let poppins = assets.fonts.get(&LFont::FNT_POPPINS).unwrap();
        let poppins_black = assets.fonts.get(&LFont::FNT_POPPINS_BLACK).unwrap();
        let roundness = 0.2;

        #[cfg(debug_assertions)]
        {
            d.draw_line(
                0,
                WINDOW_CENTER_Y as i32,
                WINDOW_WIDTH,
                WINDOW_CENTER_Y as i32,
                Color::BLUE,
            );

            d.draw_line(
                WINDOW_CENTER_X as i32,
                0,
                WINDOW_CENTER_X as i32,
                WINDOW_HEIGHT,
                Color::BLUE,
            );
        }

        if local_player.ready && local_player.is_alive() {
            // health bar
            d.draw_rectangle_rounded(
                Rectangle::new(WINDOW_PADDING as f32, WINDOW_PADDING as f32, 200.0, 20.0),
                roundness,
                0,
                Color::new(0, 0, 0, 100),
            );

            d.draw_rectangle_rounded(
                Rectangle::new(
                    WINDOW_PADDING as f32,
                    WINDOW_PADDING as f32,
                    (local_player.health as f32) / (ENTITY_PLAYER_MAX_HEALTH as f32) * 200.0,
                    20.0,
                ),
                roundness,
                1,
                Color::new(42, 192, 138, 255),
            );

            // cash
            let cash = format!("${}", local_player.inventory.cash);
            d.draw_text_ex(
                &poppins_black,
                &cash,
                RVector2::new(WINDOW_PADDING as f32, WINDOW_PADDING as f32 * 2.0),
                32.0,
                1.0,
                Color::WHITE,
            );

            let weapon_icon_length = 50.0;
            let bl_window_y = WINDOW_BOTTOM_LEFT_Y as f32 - WINDOW_PADDING as f32;

            // wapon
            if let Some(selected_wpn) = local_player.inventory.selected_weapon() {
                #[cfg(debug_assertions)]
                {
                    let data = format!("{:#?}", local_player.inventory.weapons.len());
                    d.draw_text_ex(
                        &poppins,
                        &data,
                        RVector2::new(WINDOW_PADDING as f32, WINDOW_PADDING as f32 * 3.0),
                        22.0,
                        1.0,
                        Color::RED,
                    );

                    d.draw_text_ex(
                        &poppins,
                        &local_player.reloading.to_string(),
                        RVector2::new(WINDOW_PADDING as f32 * 8.0, WINDOW_PADDING as f32 * 2.0),
                        24.0,
                        1.0,
                        Color::RED,
                    )
                }

                let font_size = 32.0;
                let mut color = Color::WHITE;
                let mut text = format!(
                    "{}/{}",
                    selected_wpn.curr_mag_ammo, selected_wpn.curr_total_ammo
                );

                let text_size = text::measure_text_ex(&poppins_black, &text, 32.0, 1.0);

                if selected_wpn.is_empty() {
                    text = String::from("RELOAD!");
                    color = Color::RED;
                }

                if local_player.reloading {
                    text = String::from("Reloading...");
                    color = Color::GRAY;
                }

                d.draw_text_ex(
                    &poppins_black,
                    &text,
                    RVector2::new(
                        WINDOW_PADDING as f32,
                        bl_window_y - weapon_icon_length - text_size.y * 1.1,
                    ),
                    font_size,
                    1.0,
                    color,
                );

                // waspons ui
                let lock_icon = assets.textures.get(&LTexture::UI_LOCK).unwrap();
                for (index, wpn_variant) in WeaponVariant::VARIANTS.iter().enumerate() {
                    let texture = match wpn_variant {
                        WeaponVariant::DEAN_1911 => assets.textures.get(&LTexture::UI_DEAN_1911),
                        WeaponVariant::AKA_69 => assets.textures.get(&LTexture::UI_AKA_69),
                        WeaponVariant::SHOTPEW => assets.textures.get(&LTexture::UI_SHOTPEW),
                        WeaponVariant::PRRR => assets.textures.get(&LTexture::UI_PRRR),
                    };

                    let other_wpn = Weapon::new(*wpn_variant);

                    if let Some(buffer) = texture {
                        let dest_rect = Rectangle::new(
                            WINDOW_PADDING as f32 + (index * 60) as f32,
                            bl_window_y - weapon_icon_length,
                            weapon_icon_length,
                            weapon_icon_length,
                        );

                        let affordable =
                            local_player.inventory.cash >= *other_wpn.stats.price() as i64;

                        d.draw_texture_pro(
                            buffer,
                            Rectangle::new(0.0, 0.0, buffer.width as f32, buffer.height as f32),
                            dest_rect,
                            RVector2::zero(),
                            0.0,
                            if selected_wpn.variant == other_wpn.variant {
                                Color::WHITE
                            } else {
                                if !affordable {
                                    Color::BLACK
                                } else {
                                    Color::GRAY
                                }
                            },
                        );

                        if !local_player.inventory.has(&other_wpn.variant) {
                            d.draw_texture_pro(
                                lock_icon,
                                Rectangle::new(
                                    0.0,
                                    0.0,
                                    lock_icon.width as f32,
                                    lock_icon.height as f32,
                                ),
                                dest_rect,
                                RVector2::zero(),
                                0.0,
                                Color::WHITE,
                            )
                        }
                    };
                }
            };
        }
    }
}
