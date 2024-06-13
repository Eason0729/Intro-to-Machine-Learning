mod bullet;
mod player;
mod station;
mod wall;

pub use bullet::*;
pub use player::*;
pub use station::*;
pub use wall::*;

#[derive(Debug, Clone, Default)]
pub struct Info<'a> {
    pub player: Player,
    pub teammates: &'a [Player],
    pub enemies: &'a [Player],
    pub bullets: &'a [Bullet],
    pub bullet_stations: &'a [Station],
    pub oil_stations: &'a [Station],
    pub walls: &'a [Wall],
}

#[repr(C)]
pub struct RawInfo {
    player: *const Player,
    teammates: *const Player,
    teammates_len: u32,
    enemies: *const Player,
    enemies_len: u32,
    bullets: *const Bullet,
    bullet_len: u32,
    bullet_stations: *const Station,
    bullet_stations_len: u32,
    oil_stations: *const Station,
    oil_stations_len: u32,
    walls: *const Wall,
    walls_len: u32,
}

impl<'a> Info<'a> {
    pub unsafe fn from_raw(self_: *const RawInfo) -> Self {
        let raw = &*self_;
        Info {
            player: (&*raw.player).clone(),
            teammates: std::slice::from_raw_parts(raw.teammates, raw.teammates_len as usize),
            enemies: std::slice::from_raw_parts(raw.enemies, raw.enemies_len as usize),
            bullets: std::slice::from_raw_parts(raw.bullets, raw.bullet_len as usize),
            bullet_stations: std::slice::from_raw_parts(
                raw.bullet_stations,
                raw.bullet_stations_len as usize,
            ),
            oil_stations: std::slice::from_raw_parts(
                raw.oil_stations,
                raw.oil_stations_len as usize,
            ),
            walls: std::slice::from_raw_parts(raw.walls, raw.walls_len as usize),
        }
    }
}
