use std::thread;
use std::time::Duration;

use image::Rgb;
use rdev::Key;

use crate::coords::Position;
use crate::input::send_key;
use crate::pixel;
use crate::pixel::get_pixel_rgb;

/// Kills `quantity` monsters in Adventure Mode, by using Regular Attacks. Will disable Idle Mode if needed.
pub fn kill_monsters(quantity: u16) {
    if is_idle_mode() {
        send_key(Key::KeyQ); // Disable Idle Mode
    }

    for kills in 1..=quantity {
        while !is_enemy_alive() {
            thread::sleep(Duration::from_millis(50));
        }

        while is_enemy_alive() {
            attack_highest_available();
            thread::sleep(Duration::from_millis(50));
        }
        // It's possible that the monster is still alive, but we can not see it
        // because the bar is almost completely white
        thread::sleep(Duration::from_millis(1050));
        attack(); // So we attack an extra time
        println!("[LOG] Kill Counter: {kills}");
    }
}

pub fn kill_bosses(quantity: u16) {
    if is_idle_mode() {
        send_key(Key::KeyQ); // Disable Idle Mode
    }

    let mut kill_counter = 0;
    while kill_counter < quantity {
        while !is_enemy_alive() {
            thread::sleep(Duration::from_millis(50));
        }

        if is_enemy_boss() {
            println!("[LOG] Found a boss, fighting.");
        } else {
            println!("[LOG] Not a boss, skipping.");
            refresh_zone();
            continue;
        }

        while is_enemy_alive() {
            attack_highest_available();
            thread::sleep(Duration::from_millis(50));
        }
        // It's possible that the monster is still alive, but we can not see it
        // because the bar is almost completely white
        thread::sleep(Duration::from_millis(1050));
        attack(); // So we attack an extra time
        kill_counter += 1;
        println!("[LOG] Kill Counter: {kill_counter}");
    }
}

fn refresh_zone() {
    send_key(Key::LeftArrow);
    send_key(Key::RightArrow);
}

fn attack_highest_available() {
    // Attempt to cast all skills, stronger first
    // This (generally) amounts to using the strongest skill available
    // TODO: refactor this to
    //       1 - Know which ones are in cooldown (get button pixel color)
    //       2 - Use skills in order for maximum effectiveness

    UltimateBuff.cast();
    OffensiveBuff.cast();
    Charge.cast();
    UltimateAttack.cast();
    PiercingAttack.cast();
    Parry.cast();
    StrongAttack.cast();
    RegularAttack.cast();

    // Defensive skills are not needed right now
    // Block.cast();
    // DefensiveBuff.cast();
    // Heal.cast();
}

fn attack() {
    send_key(Key::KeyW); // Regular attack
}

fn is_enemy_alive() -> bool {
    let color = pixel::get_pixel_rgb(pixel::ENEMY_BAR_LEFT_PIXEL.into());
    color == pixel::ENEMY_ALIVE_RGB
}

fn is_enemy_boss() -> bool {
    let color = pixel::get_pixel_rgb(Pixels::BOSS_CROWN.into());
    color == Colors::BOSS_CROWN_RGB
}

fn is_idle_mode() -> bool {
    let color = pixel::get_pixel_rgb(pixel::IDLE_MODE_PIXEL.into());
    color == pixel::IDLE_MODE_ON_RGB
}

/// Adventure mode skill keys.
#[non_exhaustive]
struct Keys;

impl Keys {
    const REGULAR_ATTACK: Key = Key::KeyW;
    const STRONG_ATTACK: Key = Key::KeyE;
    const PARRY: Key = Key::KeyR;
    const PIERCING_ATTACK: Key = Key::KeyT;
    const ULTIMATE_ATTACK: Key = Key::KeyY;

    const BLOCK: Key = Key::KeyA;
    const DEFENSIVE_BUFF: Key = Key::KeyS;
    const HEAL: Key = Key::KeyD;
    const OFFENSIVE_BUFF: Key = Key::KeyF;
    const CHARGE: Key = Key::KeyG;
    const ULTIMATE_BUFF: Key = Key::KeyH;
}

struct AdventureSkill {
    key: Key,
    pixel_coords: Position,
    row_number: u8,
}

impl AdventureSkill {
    const fn new(key: Key, pixel_coords: Position, row_number: u8) -> Self {
        Self {
            key,
            pixel_coords,
            row_number,
        }
    }
}

struct Pixels;

impl Pixels {
    const REGULAR_ATTACK: Position = Position::from_coords(620, 128);
    const STRONG_ATTACK: Position = Position::from_coords(768, 128);
    const PARRY: Position = Position::from_coords(906, 128);
    const PIERCING_ATTACK: Position = Position::from_coords(1051, 128);
    const ULTIMATE_ATTACK: Position = Position::from_coords(1189, 128);

    const BLOCK: Position = Position::from_coords(485, 175);
    const DEFENSIVE_BUFF: Position = Position::from_coords(631, 128);
    const HEAL: Position = Position::from_coords(766, 128);
    const OFFENSIVE_BUFF: Position = Position::from_coords(910, 128);
    const CHARGE: Position = Position::from_coords(1050, 128);
    const ULTIMATE_BUFF: Position = Position::from_coords(1190, 128);

    const BOSS_CROWN: Position = Position::from_coords(986, 377);
}

struct Colors;

impl Colors {
    const FIRST_ROW_COOLDOWN_RGB: Rgb<u8> = Rgb([124, 78, 78]);
    const SECOND_ROW_COOLDOWN_RGB: Rgb<u8> = Rgb([51, 68, 82]);
    const BOSS_CROWN_RGB: Rgb<u8> = Rgb([247, 239, 41]);
}

impl Skill for AdventureSkill {
    fn is_available(&self) -> bool {
        let current_color = get_pixel_rgb(self.pixel_coords.into());
        match self.row_number {
            1 => current_color != Colors::FIRST_ROW_COOLDOWN_RGB,
            2 => current_color != Colors::SECOND_ROW_COOLDOWN_RGB,
            _ => panic!("Unexpected row number"),
        }
    }

    fn cast(&self) -> bool {
        if !self.is_available() {
            return false;
        }

        send_key(self.key);
        true
    }
}

const RegularAttack: AdventureSkill =
    AdventureSkill::new(Keys::REGULAR_ATTACK, Pixels::REGULAR_ATTACK, 1);

const StrongAttack: AdventureSkill =
    AdventureSkill::new(Keys::STRONG_ATTACK, Pixels::STRONG_ATTACK, 1);

const Parry: AdventureSkill = AdventureSkill::new(Keys::PARRY, Pixels::PARRY, 1);

const PiercingAttack: AdventureSkill =
    AdventureSkill::new(Keys::PIERCING_ATTACK, Pixels::PIERCING_ATTACK, 1);

const UltimateAttack: AdventureSkill =
    AdventureSkill::new(Keys::ULTIMATE_ATTACK, Pixels::ULTIMATE_ATTACK, 1);

const Block: AdventureSkill = AdventureSkill::new(Keys::BLOCK, Pixels::BLOCK, 2);

const DefensiveBuff: AdventureSkill =
    AdventureSkill::new(Keys::DEFENSIVE_BUFF, Pixels::DEFENSIVE_BUFF, 2);

const Heal: AdventureSkill = AdventureSkill::new(Keys::HEAL, Pixels::HEAL, 2);

const OffensiveBuff: AdventureSkill =
    AdventureSkill::new(Keys::OFFENSIVE_BUFF, Pixels::OFFENSIVE_BUFF, 2);

const Charge: AdventureSkill = AdventureSkill::new(Keys::CHARGE, Pixels::CHARGE, 2);

const UltimateBuff: AdventureSkill =
    AdventureSkill::new(Keys::ULTIMATE_BUFF, Pixels::ULTIMATE_BUFF, 2);

trait Skill {
    /// Returns true if skill is currently available to be used, false otherwise.
    fn is_available(&self) -> bool;

    /// Attempts to cast the skill. Returns true if cast was successful.
    ///
    /// A cast is successful if the skill was ready (i.e is_available() is true).
    /// Otherwise, the cast fails and nothing happens.
    fn cast(&self) -> bool;
}
