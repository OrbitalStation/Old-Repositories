#[derive(Debug, Clone)]
pub struct True;

#[derive(Debug, Clone)]
pub struct False;

pub trait Boolean {}

impl Boolean for True {}

impl Boolean for False {}
