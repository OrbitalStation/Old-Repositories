use std::io::Read;
use crate::field::Field;

pub fn on_ctrl_s() {
    std::fs::write("saves/save.dat", unsafe { core::mem::transmute::<(&'static Field, usize), &[u8]>((crate::field::field(), core::mem::size_of::<Field>())) }).unwrap()
}

pub fn on_ctrl_l() {
    std::fs::File::open("saves/save.dat").unwrap().read_exact(unsafe { core::mem::transmute((crate::field::field(), core::mem::size_of::<Field>())) }).unwrap()
}
