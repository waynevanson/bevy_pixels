use bevy::prelude::*;
use pixels::Pixels;

/// Wrapper component for underlying [`Pixels`] struct.
#[derive(Component, Debug)]
pub struct PixelsWrapper<'win> {
    pub pixels: Pixels<'win>,
}
