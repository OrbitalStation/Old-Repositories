use crate::{plant, tick};
use std::{
    fs::{OpenOptions, File},
    io::prelude::*
};
use core::mem::size_of;

pub struct ActionsToReach {
    eat: u8
}

impl ActionsToReach {
    pub const fn stub() -> Self {
        Self { eat: 0 }
    }
}

pub struct HerbariumBook {
    plants: [ActionsToReach; plant::Type::Count as usize],
    updated: bool
}

impl HerbariumBook {
    pub const fn stub() -> Self {
        const ATR: ActionsToReach = ActionsToReach::stub();

        Self {
            plants: [ATR; plant::Type::Count as usize],
            updated: true
        }
    }

    fn reset_to_default(&mut self) {
        let mut i = plant::Type::from(0);

        while i != plant::Type::Count {
            self.plants[i as usize].eat = i.eat_to_explore();
            i = plant::Type::from(i as u8 + 1)
        }
    }

    fn load_from_file(&mut self, file: &mut File) {
        let n_plants = crate::read_u8(file, crate::BOOK_HERBARIUM_PATH);
        let data_size = crate::read_u8(file, crate::BOOK_HERBARIUM_PATH);

        if (data_size > size_of::<ActionsToReach>() as u8) || (n_plants > self.plants.len() as u8) {
            panic!("Using old version for recognizing content of {}, which version is newer", crate::BOOK_HERBARIUM_PATH)
        }

        self.reset_to_default();

        let mut i = 0;
        while i < n_plants {
            let mut ptr = (&mut self.plants[i as usize]) as *mut ActionsToReach as *mut u8;
            let mut j = 0;
            while j < data_size {
                unsafe { *ptr = crate::read_u8(file, crate::BOOK_HERBARIUM_PATH) }
                ptr = (ptr as usize + 1) as *mut u8;
                j += 1
            }
            i += 1
        }
    }

    fn save_to_file(&self, file: &mut File) {
        let buf = [self.plants.len() as u8, size_of::<ActionsToReach>() as u8];
        crate::write_bytes(file, crate::BOOK_HERBARIUM_PATH, buf.as_slice());

        let mut buf = [0u8; size_of::<ActionsToReach>()];
        let mut i = 0;
        while i < self.plants.len() {
            let mut ptr = (&self.plants[i]) as *const ActionsToReach as *const u8;
            let mut j = 0;
            while j < size_of::<ActionsToReach>() {
                unsafe { buf[j] = *ptr }
                ptr = (ptr as usize + 1) as *const u8;
                j += 1
            }
            crate::write_bytes(file, crate::BOOK_HERBARIUM_PATH, buf.as_slice());
            i += 1
        }
    }

    pub fn save(&mut self) {
        if self.updated {
            match OpenOptions::new().truncate(true).write(true).create(true).open(crate::BOOK_HERBARIUM_PATH) {
                Ok(mut file) => self.save_to_file(&mut file),
                Err(err) => panic!("Failed to open {} to save data:\n\t{}", crate::BOOK_HERBARIUM_PATH, err)
            }
            self.updated = false;
        }
    }

    pub fn init(&mut self) {
        match OpenOptions::new().read(true).open(crate::BOOK_HERBARIUM_PATH) {
            Ok(mut file) => self.load_from_file(&mut file), //< If file exists, we read data from it
            Err(_) => self.reset_to_default()                    //< If file doesn't exist, we set data to default values
        }
        tick::add(|_| {
            unsafe { HERBARIUM.save() }
            crate::BOOK_HERBARIUM_SAVE_IN
        }, tick::NULLARG, crate::BOOK_HERBARIUM_SAVE_IN);
    }
}

pub static mut HERBARIUM: HerbariumBook = HerbariumBook::stub();
