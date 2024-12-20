use std::ops::{Add, Neg, Sub};

// Macros below are copied from https://github.com/rust-lang/rust/blob/8a1f8039a7ded79d3d4fe97b110016d89f2b11e2/library/core/src/internal_macros.rs#L3-L62
macro_rules! forward_ref_unop {
	(impl $imp:ident, $method:ident for $t:ty) => {
		forward_ref_unop!(impl $imp, $method for $t,
				#[stable(feature = "rust1", since = "1.0.0")]);
	};
	(impl $imp:ident, $method:ident for $t:ty, #[$attr:meta]) => {
		impl $imp for &$t {
			type Output = <$t as $imp>::Output;

			#[inline]
			fn $method(self) -> <$t as $imp>::Output {
				$imp::$method(*self)
			}
		}
	}
}

macro_rules! forward_ref_binop {
	(impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
		forward_ref_binop!(impl $imp, $method for $t, $u,
				#[stable(feature = "rust1", since = "1.0.0")]);
	};
	(impl $imp:ident, $method:ident for $t:ty, $u:ty, #[$attr:meta]) => {
		impl<'a> $imp<$u> for &'a $t {
			type Output = <$t as $imp<$u>>::Output;

			#[inline]
			#[track_caller]
			fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
				$imp::$method(*self, other)
			}
		}

		impl $imp<&$u> for $t {
			type Output = <$t as $imp<$u>>::Output;

			#[inline]
			#[track_caller]
			fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
				$imp::$method(self, *other)
			}
		}

		impl $imp<&$u> for &$t {
			type Output = <$t as $imp<$u>>::Output;

			#[inline]
			#[track_caller]
			fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
				$imp::$method(*self, *other)
			}
		}
	}
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Position {
	pub x: isize,
	pub y: isize,
}

impl Position {
	pub fn new(x: isize, y: isize) -> Self {
		Self { x, y }
	}

	pub fn manhattan_length(&self) -> isize {
		self.x.abs() + self.y.abs()
	}

	pub fn manhattan_distance(&self, other: &Position) -> isize {
		(self - other).manhattan_length()
	}
}

impl Add<Position> for Position {
	type Output = Self;

	#[inline]
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

forward_ref_binop!(impl Add, add for Position, Position);

impl Sub for Position {
	type Output = Self;

	#[inline]
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

forward_ref_binop!(impl Sub, sub for Position, Position);

impl Neg for Position {
	type Output = Self;

	#[inline]
	fn neg(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
		}
	}
}

forward_ref_unop!(impl Neg, neg for Position);

impl std::fmt::Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}
