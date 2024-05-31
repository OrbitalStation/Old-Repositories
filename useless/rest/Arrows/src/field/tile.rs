use sfml::{
    graphics::{Texture, Sprite, IntRect, Transformable},
    SfBox
};

/*macro_rules! tile {
    ('ord: $($ord:ident = $v1:literal)* 'dat: $($dat:ident($($t:ty = $d:expr),*) = $v2:literal)*) => {
        #[derive(Copy, Clone, Eq, PartialEq)]
        #[repr(u8)]
        pub enum TileType {
            $($ord,)*
            $($dat($($t)*),)*

            Count
        }

        impl TileType {
            pub fn as_u8(self) -> u8 {
                match self {
                    $(Self::$ord => $v1,)*
                    $(Self::$dat(_) => $v2,)*

                    Self::Count => unimplemented!()
                }
            }

            pub fn from_u8(x: u8) -> Self {
                match x {
                    $($v1 => Self::$ord,)*
                    $($v2 => Self::$dat($($d),*),)*
                    _ => Self::Count
                }
            }
        }
    };
}*/


#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum TileType {
    Void,

    Arrow,
    ActivatedArrow,

    LongArrow,
    ActivatedLongArrow,

    VeryLongArrow,
    ActivatedVeryLongArrow,

    Generator,

    Not,
    ActivatedNot,

    And,

    DoubleAngleArrow,
    ActivatedDoubleAngleArrow,

    DoubleStraightArrow,
    ActivatedDoubleStraightArrow,

    EmptyRepeater,
    ReadyRepeater,
    ActivatedRepeater,

    Toggle,
    ActivatedToggle,

    Count
}

impl TileType {
    pub fn is_activated(self) -> bool {
        matches!(self, Self::ActivatedArrow | Self::ActivatedLongArrow | Self::ActivatedVeryLongArrow | Self::Generator | Self::ActivatedNot | Self::ActivatedDoubleAngleArrow | Self::ActivatedDoubleStraightArrow | Self::ActivatedRepeater | Self::ActivatedToggle)
    }

    pub fn is_rotatable(self) -> bool {
        matches!(self, Self::Arrow | Self::ActivatedArrow | Self::LongArrow | Self::ActivatedLongArrow | Self::VeryLongArrow | Self::ActivatedVeryLongArrow | Self::And | Self::DoubleAngleArrow | Self::ActivatedDoubleAngleArrow | Self::DoubleStraightArrow | Self::ActivatedDoubleStraightArrow | Self::ReadyRepeater | Self::EmptyRepeater | Self::ActivatedRepeater)
    }

    //noinspection DuplicatedCode
    pub fn activate(&mut self) {
        *self = match *self {
            Self::Arrow => Self::ActivatedArrow,
            Self::Not => Self::ActivatedNot,
            Self::DoubleAngleArrow => Self::ActivatedDoubleAngleArrow,
            Self::DoubleStraightArrow => Self::ActivatedDoubleStraightArrow,
            Self::EmptyRepeater => Self::ReadyRepeater,
            Self::ReadyRepeater => Self::ActivatedRepeater,
            Self::ActivatedRepeater => Self::EmptyRepeater,
            Self::LongArrow => Self::ActivatedLongArrow,
            Self::VeryLongArrow => Self::ActivatedVeryLongArrow,
            _ => *self
        }
    }

    //noinspection DuplicatedCode
    pub fn deactivate(&mut self) {
        *self = match *self {
            Self::ActivatedArrow => Self::Arrow,
            Self::ActivatedNot => Self::Not,
            Self::ActivatedDoubleAngleArrow => Self::DoubleAngleArrow,
            Self::ActivatedDoubleStraightArrow => Self::DoubleStraightArrow,
            Self::ReadyRepeater => Self::ActivatedRepeater,
            Self::ActivatedRepeater => Self::EmptyRepeater,
            Self::ActivatedLongArrow => Self::LongArrow,
            Self::ActivatedVeryLongArrow => Self::VeryLongArrow,
            _ => *self
        }
    }

    fn is_ignorable_by_next(self) -> bool {
        matches!(self, Self::ActivatedArrow | Self::ActivatedLongArrow | Self::ActivatedVeryLongArrow | Self::ActivatedNot | Self::ActivatedDoubleAngleArrow | Self::ActivatedDoubleStraightArrow | Self::ReadyRepeater | Self::ActivatedRepeater | Self::ActivatedToggle)
    }

    pub fn next(&mut self) {
        loop {
            *self = unsafe { core::mem::transmute(*self as u8 + 1) };
            if *self == Self::Count { *self = Self::Void }
            if !self.is_ignorable_by_next() { break }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Rotation {
    Up,
    Right,
    Down,
    Left,

    Count
}

impl Rotation {
    pub fn degrees(self) -> f32 {
        match self {
            Self::Up => 0.,
            Self::Right => 90.,
            Self::Down => 180.,
            Self::Left => 270.,
            _ => unimplemented!()
        }
    }

    pub fn next(&mut self) {
        *self = unsafe { core::mem::transmute(*self as u8 + 1) };
        if *self == Rotation::Count { *self = Rotation::Up }
    }
}

impl core::ops::Not for Rotation {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
            _ => unimplemented!()
        }
    }
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub ty: TileType,
    pub rot: Rotation,
    pub was_activated: bool
}

impl Tile {
    pub const VOID: Self = Self::new(TileType::Void, Rotation::Up);

    pub const SS: f32 = SIZE as f32;
    pub const HSS: f32 = Self::SS / 2.;

    #[inline]
    pub const fn new(ty: TileType, rot: Rotation) -> Self {
        Self { ty, rot, was_activated: false }
    }

    pub fn load(filename: &str) {
        unsafe {
            TEXTURE = Some(Texture::from_file(filename).unwrap());

            let mut sprite = Sprite::with_texture(TEXTURE.as_ref().unwrap());
            sprite.set_origin((Self::HSS, Self::HSS));

            SPRITE = Some(sprite);
        }
    }

    pub fn from_raw(x: i32, y: i32) -> &'static mut Self {
        &mut super::field()[(x / Self::SS as i32) as usize][(y / Self::SS as i32) as usize]
    }

    pub fn sprite() -> &'static mut Sprite <'static> {
        unsafe { SPRITE.as_mut().unwrap() }
    }

    pub fn update_sprite(&self, x: f32, y: f32) {
        Self::sprite().set_position(sfml::system::Vector2f::new(x * Tile::SS + Self::HSS, y * Tile::SS + Self::HSS));
        Self::sprite().set_texture_rect(&IntRect::new(self.ty as i32 * SIZE, 0, SIZE, SIZE));
        Self::sprite().set_rotation(self.rot.degrees())
    }

    pub fn update_ty(&mut self) {
        self.ty.next()
    }

    pub fn update_rot(&mut self) {
        if self.ty == TileType::Toggle { self.ty = TileType::ActivatedToggle }
        else if self.ty == TileType::ActivatedToggle { self.ty = TileType::Toggle }
        else if self.ty.is_rotatable() { self.rot.next() }
    }

    pub fn try_activate(&mut self, not: Rotation) {
        if self.ty == TileType::And { return }
        if self.rot != not { self.was_activated = true }
    }
}

const SIZE: i32 = 20;

static mut TEXTURE: Option <SfBox <Texture>> = None;
static mut SPRITE: Option <Sprite <'static>> = None;
