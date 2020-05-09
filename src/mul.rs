use crate::Polynomial;

use core::convert::TryFrom;
use core::ops::{AddAssign, Mul, MulAssign};
use num::{One, Zero};
use smallvec::{smallvec, SmallVec};

// originally wanted to implement these fully generically, i.e.
//
// impl<L, R, O> Mul<&Polynomial<R>> for &Polynomial<L>
// where
//     for<'l, 'r> &'l L: Mul<&'r R, Output = O>,
//     O: Zero + AddAssign,
// {}
//
// etc. but this causes issues with scalar multiplication:
// https://gist.github.com/l0calh05t/b56b39cd9594e1e3e813c8ab9026f0df
//

impl<T> Mul<&Polynomial<T>> for &Polynomial<T>
where
	T: Zero + AddAssign,
	for<'l, 'r> &'l T: Mul<&'r T, Output = T>,
{
	type Output = Polynomial<T>;
	fn mul(self, rhs: &Polynomial<T>) -> Polynomial<T> {
		let order_l = self.order();
		let order_r = rhs.order();
		let order_o = order_l.checked_add(order_r).unwrap();
		let mut rev_coeffs: SmallVec<[T; 8]> = SmallVec::new();

		let num_add = usize::try_from(order_o).unwrap().checked_add(1).unwrap();
		rev_coeffs.reserve(num_add);
		for _ in 0..num_add {
			rev_coeffs.push(T::zero());
		}

		/*rev_coeffs.resize_with(
			usize::try_from(order_o).unwrap().checked_add(1).unwrap(),
			T::zero,
		);*/

		for (el, vl) in self.rev_coeffs.iter().enumerate() {
			for (er, vr) in rhs.rev_coeffs.iter().enumerate() {
				*unsafe {
					// the following line causes clippy to emit a spurious warning (using + in a Mul-implementation)
					// oddly enough, the later += doesn't trigger a warning
					#[allow(clippy::suspicious_arithmetic_impl)]
					rev_coeffs.get_unchecked_mut(el + er)
				} += vl * vr;
			}
		}

		Polynomial::new_reversed(rev_coeffs)
	}
}

impl<T> Mul<&Polynomial<T>> for Polynomial<T>
where
	T: Zero + AddAssign,
	for<'l, 'r> &'l T: Mul<&'r T, Output = T>,
{
	type Output = Polynomial<T>;
	fn mul(self, rhs: &Polynomial<T>) -> Polynomial<T> {
		&self * rhs
	}
}

impl<T> Mul<Polynomial<T>> for &Polynomial<T>
where
	T: Zero + AddAssign,
	for<'l, 'r> &'l T: Mul<&'r T, Output = T>,
{
	type Output = Polynomial<T>;
	fn mul(self, rhs: Polynomial<T>) -> Polynomial<T> {
		self * &rhs
	}
}

impl<T> Mul<Polynomial<T>> for Polynomial<T>
where
	T: Zero + AddAssign,
	for<'l, 'r> &'l T: Mul<&'r T, Output = T>,
{
	type Output = Polynomial<T>;
	fn mul(self, rhs: Polynomial<T>) -> Polynomial<T> {
		&self * &rhs
	}
}

impl<T> MulAssign<Polynomial<T>> for Polynomial<T>
where
	T: Zero + AddAssign,
	for<'l, 'r> &'l T: Mul<&'r T, Output = T>,
{
	fn mul_assign(&mut self, rhs: Polynomial<T>) {
		let val = &*self * &rhs;
		*self = val;
	}
}

impl<T> MulAssign<&Polynomial<T>> for Polynomial<T>
where
	T: Zero + AddAssign,
	for<'l, 'r> &'l T: Mul<&'r T, Output = T>,
{
	fn mul_assign(&mut self, rhs: &Polynomial<T>) {
		let val = &*self * rhs;
		*self = val;
	}
}

impl<T> Mul<&T> for Polynomial<T>
where
	T: for<'r> Mul<&'r T, Output = T> + Zero,
{
	type Output = Polynomial<T>;
	fn mul(self, rhs: &T) -> Polynomial<T> {
		Polynomial::new_reversed(self.rev_coeffs.into_iter().map(|lhs| lhs * rhs).collect())
	}
}

impl<T> Mul<T> for Polynomial<T>
where
	T: for<'r> Mul<&'r T, Output = T> + Zero,
{
	type Output = Polynomial<T>;
	fn mul(self, rhs: T) -> Polynomial<T> {
		self * &rhs
	}
}

impl<T> Mul<&T> for &Polynomial<T>
where
	T: Zero,
	for<'l, 'r> &'l T: Mul<&'r T, Output = T>,
{
	type Output = Polynomial<T>;
	fn mul(self, rhs: &T) -> Polynomial<T> {
		Polynomial::new_reversed(self.rev_coeffs.iter().map(|lhs| lhs * rhs).collect())
	}
}

impl<T> Mul<T> for &Polynomial<T>
where
	T: Zero,
	for<'l, 'r> &'l T: Mul<&'r T, Output = T>,
{
	type Output = Polynomial<T>;
	fn mul(self, rhs: T) -> Polynomial<T> {
		self * &rhs
	}
}

/*impl<T> Mul<&Polynomial<T>> for T
where
	T: Zero,
	for<'r, 'l> &'l T: Mul<&'r T, Output = T>,
{
	type Output = Polynomial<T>;
	fn mul(self, rhs: &Polynomial<T>) -> Polynomial<T> {
		Polynomial::new_reversed(rhs.rev_coeffs.iter().map(|rhs| &self * rhs).collect())
	}
}*/

macro_rules! implement_left_scalar_mul {
	($T:path) => {
		impl Mul<&Polynomial<$T>> for $T {
			type Output = Polynomial<$T>;
			fn mul(self, rhs: &Polynomial<$T>) -> Polynomial<$T> {
				Polynomial::new_reversed(rhs.rev_coeffs.iter().map(|v| &self * v).collect())
			}
		}

		impl Mul<&Polynomial<$T>> for &$T {
			type Output = Polynomial<$T>;
			fn mul(self, rhs: &Polynomial<$T>) -> Polynomial<$T> {
				Polynomial::new_reversed(rhs.rev_coeffs.iter().map(|v| self * v).collect())
			}
		}

		impl Mul<Polynomial<$T>> for $T {
			type Output = Polynomial<$T>;
			fn mul(self, rhs: Polynomial<$T>) -> Polynomial<$T> {
				let mut rhs = rhs;
				for v in rhs.rev_coeffs.iter_mut() {
					*v *= &self;
				}
				rhs.fixup_coefficients();
				rhs
			}
		}

		impl Mul<Polynomial<$T>> for &$T {
			type Output = Polynomial<$T>;
			fn mul(self, rhs: Polynomial<$T>) -> Polynomial<$T> {
				let mut rhs = rhs;
				for v in rhs.rev_coeffs.iter_mut() {
					*v *= self;
				}
				rhs.fixup_coefficients();
				rhs
			}
		}
	};
}

implement_left_scalar_mul!(i8);
implement_left_scalar_mul!(i16);
implement_left_scalar_mul!(i32);
implement_left_scalar_mul!(i64);
implement_left_scalar_mul!(isize);
implement_left_scalar_mul!(num::BigInt);

implement_left_scalar_mul!(u8);
implement_left_scalar_mul!(u16);
implement_left_scalar_mul!(u32);
implement_left_scalar_mul!(u64);
implement_left_scalar_mul!(usize);
implement_left_scalar_mul!(num::BigUint);

implement_left_scalar_mul!(f32);
implement_left_scalar_mul!(f64);

implement_left_scalar_mul!(num::Complex<i8>);
implement_left_scalar_mul!(num::Complex<i16>);
implement_left_scalar_mul!(num::Complex<i32>);
implement_left_scalar_mul!(num::Complex<i64>);
implement_left_scalar_mul!(num::Complex<isize>);

implement_left_scalar_mul!(num::Complex<u8>);
implement_left_scalar_mul!(num::Complex<u16>);
implement_left_scalar_mul!(num::Complex<u32>);
implement_left_scalar_mul!(num::Complex<u64>);
implement_left_scalar_mul!(num::Complex<usize>);

implement_left_scalar_mul!(num::Complex<f32>);
implement_left_scalar_mul!(num::Complex<f64>);

implement_left_scalar_mul!(num::Rational);
implement_left_scalar_mul!(num::rational::Rational32);
implement_left_scalar_mul!(num::rational::Rational64);
implement_left_scalar_mul!(num::BigRational);

impl<T> One for Polynomial<T>
where
	T: One + Zero + AddAssign,
	for<'l, 'r> &'l T: Mul<&'r T, Output = T>,
{
	fn one() -> Self {
		Self {
			rev_coeffs: smallvec![T::one()],
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn test_mul() {
		let a = Polynomial::new(coefficients![1f32, 3.0, 3.0, 0.0]);
		let b = Polynomial::new(coefficients![1f32, 0.0, 1.0]);

		let expected = coefficients![1f32, 3.0, 4.0, 3.0, 3.0, 0.0];

		// Polynomial * Polynomial
		let c = a.clone() * b.clone();
		assert_eq!(c.order(), 5);
		assert_eq!(c.coeffs(), expected);

		// Polynomial * Polynomial (swapped)
		let c = b.clone() * a.clone();
		assert_eq!(c.order(), 5);
		assert_eq!(c.coeffs(), expected);

		// Polynomial * &Polynomial
		let c = a.clone() * &b;
		assert_eq!(c.order(), 5);
		assert_eq!(c.coeffs(), expected);

		// &Polynomial * Polynomial
		let c = &a * b.clone();
		assert_eq!(c.order(), 5);
		assert_eq!(c.coeffs(), expected);

		// &Polynomial * &Polynomial
		let c = &a * &b;
		assert_eq!(c.order(), 5);
		assert_eq!(c.coeffs(), expected);

		// Removal of leading zeros
		let c = a.clone() * Polynomial::zero();
		assert_eq!(c.order(), 0);
		assert_eq!(c.coeffs(), coefficients![0f32]);
	}

	#[test]
	fn test_mul_assign() {
		let a = Polynomial::new(coefficients![1f32, 3.0, 3.0, 0.0]);
		let b = Polynomial::new(coefficients![1f32, 0.0, 1.0]);

		let expected = coefficients![1f32, 3.0, 4.0, 3.0, 3.0, 0.0];

		// Polynomial *= Polynomial
		let mut c = a.clone();
		c *= b.clone();
		assert_eq!(c.order(), 5);
		assert_eq!(c.coeffs(), expected);

		// Polynomial *= &Polynomial
		let mut c = a.clone();
		c *= &b;
		assert_eq!(c.order(), 5);
		assert_eq!(c.coeffs(), expected);

		// Polynomial *= Polynomial (swapped)
		let mut c = b.clone();
		c *= a.clone();
		assert_eq!(c.order(), 5);
		assert_eq!(c.coeffs(), expected);

		// Polynomial *= &Polynomial (swapped)
		let mut c = b.clone();
		c *= &a;
		assert_eq!(c.order(), 5);
		assert_eq!(c.coeffs(), expected);

		// Removal of leading zeros
		let mut c = a.clone();
		c *= Polynomial::zero();
		assert_eq!(c.order(), 0);
		assert_eq!(c.coeffs(), coefficients![0f32]);
	}

	#[test]
	fn test_mul_scalar() {
		let a = Polynomial::new(coefficients![1f32, 3.0, 3.0, 0.0]);

		let expected = coefficients![2f32, 6.0, 6.0, 0.0];

		// Polynomial * Scalar
		let b = a.clone() * 2.0;
		assert_eq!(b.order(), 3);
		assert_eq!(b.coeffs(), expected);

		// &Polynomial * Scalar
		let b = &a * 2.0;
		assert_eq!(b.order(), 3);
		assert_eq!(b.coeffs(), expected);

		// Polynomial * &Scalar
		let b = a.clone() * &2.0;
		assert_eq!(b.order(), 3);
		assert_eq!(b.coeffs(), expected);

		// &Polynomial * &Scalar
		let b = &a * &2.0;
		assert_eq!(b.order(), 3);
		assert_eq!(b.coeffs(), expected);

		// Scalar * Polynomial
		let b = 2.0 * a.clone();
		assert_eq!(b.order(), 3);
		assert_eq!(b.coeffs(), expected);

		// Scalar * &Polynomial
		let b = 2.0 * &a;
		assert_eq!(b.order(), 3);
		assert_eq!(b.coeffs(), expected);

		// &Scalar * Polynomial
		let b = &2.0 * a.clone();
		assert_eq!(b.order(), 3);
		assert_eq!(b.coeffs(), expected);

		// &Scalar * &Polynomial
		let b = &2.0 * &a;
		assert_eq!(b.order(), 3);
		assert_eq!(b.coeffs(), expected);

		// Removal of leading zeros
		let c = a.clone() * 0.0;
		assert_eq!(c.order(), 0);
		assert_eq!(c.coeffs(), coefficients![0f32]);
	}
}
