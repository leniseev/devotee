use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Generic two-dimensional vector.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vector<T> {
    x: T,
    y: T,
}

impl<T> Vector<T> {
    /// Create new vector with `x` and `y` values.
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Split this vector into its components.
    pub fn split(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T> Vector<T>
where
    T: Copy,
{
    /// Get the x value.
    pub fn x(self) -> T {
        self.x
    }

    /// Get the y value.
    pub fn y(self) -> T {
        self.y
    }
}

impl<T> Vector<T> {
    /// Get reference to the x value.
    pub const fn x_ref(&self) -> &T {
        &self.x
    }

    /// Get reference to the y value.
    pub const fn y_ref(&self) -> &T {
        &self.y
    }

    /// Get mutable reference to the x value.
    pub fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }

    /// Get mutable reference to the y value.
    pub fn y_mut(&mut self) -> &mut T {
        &mut self.y
    }

    /// Apply `mapper` function to both elements, one by one, return `Vector` with new values.
    pub fn map<F, R>(self, mapper: F) -> Vector<R>
    where
        F: Fn(T) -> R,
    {
        Vector {
            x: mapper(self.x),
            y: mapper(self.y),
        }
    }
}

impl<T> Vector<T> {
    /// Calculate the dot product between `self` and `rhs` vectors.
    pub fn dot<U, R>(self, rhs: Vector<U>) -> R
    where
        T: Mul<U, Output = R>,
        R: Add<Output = R>,
    {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T> From<(T, T)> for Vector<T> {
    fn from(source: (T, T)) -> Self {
        Self {
            x: source.0,
            y: source.1,
        }
    }
}

impl<T> From<Vector<T>> for (T, T) {
    fn from(source: Vector<T>) -> Self {
        (source.x, source.y)
    }
}

impl<T> Mul<T> for Vector<T>
where
    T: Mul<Output = T> + Clone,
{
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        Self {
            x: self.x * other.clone(),
            y: self.y * other,
        }
    }
}

impl<T> MulAssign<T> for Vector<T>
where
    T: MulAssign + Clone,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.clone();
        self.y *= rhs;
    }
}

impl<T> Div<T> for Vector<T>
where
    T: Div<Output = T> + Clone,
{
    type Output = Self;
    fn div(self, other: T) -> Self::Output {
        Self {
            x: self.x / other.clone(),
            y: self.y / other,
        }
    }
}

impl<T> DivAssign<T> for Vector<T>
where
    T: DivAssign + Clone,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs.clone();
        self.y /= rhs;
    }
}

impl<T, U> Add<U> for Vector<T>
where
    T: Add<Output = T>,
    U: Into<Vector<T>>,
{
    type Output = Self;
    fn add(self, other: U) -> Self::Output {
        let other = other.into();
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T, U> AddAssign<U> for Vector<T>
where
    T: AddAssign,
    U: Into<Vector<T>>,
{
    fn add_assign(&mut self, rhs: U) {
        let rhs = rhs.into();
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T, U> Sub<U> for Vector<T>
where
    T: Sub<Output = T>,
    U: Into<Vector<T>>,
{
    type Output = Self;
    fn sub(self, other: U) -> Self::Output {
        let other = other.into();
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T, U> SubAssign<U> for Vector<T>
where
    T: SubAssign,
    U: Into<Vector<T>>,
{
    fn sub_assign(&mut self, rhs: U) {
        let rhs = rhs.into();
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T, R> Neg for Vector<T>
where
    T: Neg<Output = R>,
{
    type Output = Vector<R>;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}
