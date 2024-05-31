mod types;
mod impls;

use crate::{
    tick,
    field::Size
};

// Public interface

pub use types::{Direction, Health, Err};

#[inline]
pub fn init() {
    load();
    tick::add(impls::state_handler_cb, tick::NULLARG, crate::PLAYER_STATE_HANDLER_IN)
}

#[inline]
pub fn generate_position() {
    unsafe { impls::generate_position_impl() }
}

#[inline]
// return --> true if move occurred, false otherwise
pub fn r#move(dir: Direction) {
    unsafe { impls::move_impl(dir, true) }
}

#[inline]
pub fn show_visible_area() {
    unsafe { impls::show_visible_area_impl() }
}

#[inline]
pub fn check() {
    unsafe { impls::check_impl() }
}

#[inline]
pub fn show_interface() {
    unsafe { impls::show_interface_impl() }
}

#[inline]
pub fn increase_satiety(value: u8) {
    unsafe { impls::change_satiety(value, false) }
}

#[inline]
pub fn decrease_satiety(value: u8) {
    unsafe { impls::change_satiety(value, true) }
}

#[inline]
pub fn increase_health(value: u8) {
    unsafe { impls::change_health(value, false) }
}

#[inline]
pub fn decrease_health(value: u8) {
    unsafe { impls::change_health(value, true) }
}

#[inline]
pub fn absorb_hunger_counter() {
    unsafe { impls::absorb_hunger_counter_impl() }
}

#[inline]
pub fn is_on(x: Size, y: Size) -> bool {
    unsafe { impls::is_on_impl(x, y) }
}

#[inline]
pub fn set_visibility(new: Size) {
    unsafe { impls::set_visibility_impl(new) }
}

#[inline]
pub fn get_visibility() -> Size {
    unsafe { impls::get_visibility_impl() }
}

#[inline]
pub fn is_visibility_updated() -> bool {
    unsafe { impls::is_visibility_updated_impl() }
}

#[inline]
pub fn load() {
    unsafe { impls::load_impl() }
}

#[inline]
pub fn save() {
    unsafe { impls::save_impl() }
}
