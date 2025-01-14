use std::slice::Iter;

use devotee_backend::BackendImage;

use super::{Image, PixelsIterator};
use crate::util::vector::Vector;

/// Canvas based on box slice of pixel data.
/// The canvas size is not known at compile time.
#[derive(Clone, Debug)]
pub struct Canvas<P> {
    data: Box<[P]>,
    width: usize,
    height: usize,
}

impl<P> Canvas<P>
where
    P: Clone,
{
    /// Create new canvas with given color and resolution.
    pub fn with_resolution(color: P, width: usize, height: usize) -> Self {
        let data = vec![color; width * height].into_boxed_slice();
        Self {
            data,
            width,
            height,
        }
    }
}

impl<P> Image for Canvas<P>
where
    P: Clone,
{
    type Pixel = P;
    type PixelRef<'a> = &'a P where P:'a;
    type PixelMut<'a> = &'a mut P where P:'a;
    fn pixel(&self, position: Vector<i32>) -> Option<&P> {
        if position.x() < 0 || position.y() < 0 {
            return None;
        }
        let (x, y) = (position.x() as usize, position.y() as usize);
        if x >= self.width || y >= self.height {
            None
        } else {
            self.data.get(x + self.width * y)
        }
    }

    fn pixel_mut(&mut self, position: Vector<i32>) -> Option<&mut P> {
        if position.x() < 0 || position.y() < 0 {
            return None;
        }
        let (x, y) = (position.x() as usize, position.y() as usize);
        if x >= self.width || y >= self.height {
            None
        } else {
            self.data.get_mut(x + self.width * y)
        }
    }

    /// Get reference to pixel.
    /// # Safety
    /// - `position` must be in range `[0, width-1]` by `x` and `[0, height-1]` by `y`.
    unsafe fn unsafe_pixel(&self, position: Vector<i32>) -> &P {
        let (x, y) = (position.x() as usize, position.y() as usize);
        &self.data[x + self.width * y]
    }

    /// Get mutable reference to pixel.
    /// # Safety
    /// - `position` must be in range `[0, width-1]` by `x` and `[0, height-1]` by `y`.
    unsafe fn unsafe_pixel_mut(&mut self, position: Vector<i32>) -> &mut P {
        let (x, y) = (position.x() as usize, position.y() as usize);
        &mut self.data[x + self.width * y]
    }

    fn width(&self) -> i32 {
        self.width as i32
    }

    fn height(&self) -> i32 {
        self.height as i32
    }

    fn clear(&mut self, color: P) {
        self.data = vec![color; self.width * self.height].into_boxed_slice();
    }
}

impl<'a, P: 'a> PixelsIterator<'a, P> for Canvas<P> {
    type Iterator = Iter<'a, P>;

    fn pixels(&'a self) -> Self::Iterator {
        self.data.iter()
    }
}

impl<'a, P: 'a> BackendImage<'a, P> for Canvas<P>
where
    P: Clone,
{
    type Iterator = Iter<'a, P>;

    unsafe fn pixel_unsafe(&self, x: u32, y: u32) -> &P {
        Image::unsafe_pixel(self, Vector::new(x as i32, y as i32))
    }

    fn width(&self) -> u32 {
        self.width as u32
    }

    fn height(&self) -> u32 {
        self.height as u32
    }

    fn pixels(&'a self) -> Self::Iterator {
        PixelsIterator::pixels(self)
    }
}
