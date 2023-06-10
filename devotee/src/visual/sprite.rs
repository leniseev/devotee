use super::{Image, PaintTarget, Painter};
use crate::util::vector::Vector;

/// Sprite of fixed dimensions.
#[derive(Clone, Copy, Debug)]
pub struct Sprite<P, const W: usize, const H: usize> {
    data: [[P; W]; H],
}

impl<P, const W: usize, const H: usize> Sprite<P, W, H>
where
    P: Copy,
{
    /// Create new Sprite with given color for each pixel.
    pub const fn with_color(color: P) -> Self {
        let data = [[color; W]; H];
        Self { data }
    }

    /// Create new Sprite with given data.
    pub const fn with_data(data: [[P; W]; H]) -> Self {
        Self { data }
    }
}

impl<P, const W: usize, const H: usize> PaintTarget<P> for Sprite<P, W, H>
where
    P: Copy,
{
    fn painter(&mut self) -> Painter<P> {
        Painter { target: self }
    }
}

impl<P, const W: usize, const H: usize> Image for Sprite<P, W, H>
where
    P: Copy,
{
    type Pixel = P;

    fn pixel(&self, position: Vector<i32>) -> Option<&P> {
        if position.x() < 0 || position.y() < 0 {
            return None;
        }
        let (x, y) = (position.x() as usize, position.y() as usize);
        if x >= W || y >= H {
            None
        } else {
            Some(&self.data[y][x])
        }
    }

    fn pixel_mut(&mut self, position: Vector<i32>) -> Option<&mut P> {
        if position.x() < 0 || position.y() < 0 {
            return None;
        }
        let (x, y) = (position.x() as usize, position.y() as usize);
        if x >= W || y >= H {
            None
        } else {
            Some(&mut self.data[y][x])
        }
    }

    unsafe fn pixel_unsafe(&self, position: Vector<i32>) -> &Self::Pixel {
        let (x, y) = (position.x() as usize, position.y() as usize);
        &self.data[y][x]
    }

    unsafe fn pixel_mut_unsafe(&mut self, position: Vector<i32>) -> &mut Self::Pixel {
        let (x, y) = (position.x() as usize, position.y() as usize);
        &mut self.data[y][x]
    }

    fn width(&self) -> i32 {
        W as i32
    }

    fn height(&self) -> i32 {
        H as i32
    }

    fn clear(&mut self, color: Self::Pixel) {
        self.data = [[color; W]; H];
    }
}

impl<P, const W: usize, const H: usize> Default for Sprite<P, W, H>
where
    P: Default + Copy,
{
    fn default() -> Self {
        Self::with_color(Default::default())
    }
}

impl<'a, P, const W: usize, const H: usize> IntoIterator for &'a Sprite<P, W, H>
where
    P: Copy,
{
    type Item = &'a P;

    type IntoIter = SpriteIter<'a, P, W, H>;

    fn into_iter(self) -> Self::IntoIter {
        SpriteIter {
            sprite: self,
            index: 0,
        }
    }
}

/// An Iterator over pixels in a sprite, left to right, top to bottom.
pub struct SpriteIter<'a, P, const W: usize, const H: usize> {
    sprite: &'a Sprite<P, W, H>,
    index: usize,
}

impl<'a, P, const W: usize, const H: usize> Iterator for SpriteIter<'a, P, W, H> {
    type Item = &'a P;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.index;
        if current < W * H {
            self.index += 1;
            Some(&self.sprite.data[current / W][current % W])
        } else {
            None
        }
    }
}
