use crate::Polynomial;

use core::ops::{Add, AddAssign};
use num::Zero;
use smallvec::smallvec;

impl<T> Add<Polynomial<T>> for Polynomial<T>
where
	T: for<'r> AddAssign<&'r T> + Zero,
{
	type Output = Polynomial<T>;

	fn add(self, rhs: Polynomial<T>) -> Polynomial<T> {
		let mut ret = self;
		ret += &rhs;
		ret
	}
}

impl<T> Add<&Polynomial<T>> for Polynomial<T>
where
	T: for<'r> AddAssign<&'r T> + Zero,
{
	type Output = Polynomial<T>;

	fn add(self, rhs: &Polynomial<T>) -> Polynomial<T> {
		let mut ret = self;
		ret += rhs;
		ret
	}
}

impl<T> Add<Polynomial<T>> for &Polynomial<T>
where
	T: for<'r> AddAssign<&'r T> + Zero,
{
	type Output = Polynomial<T>;

	fn add(self, rhs: Polynomial<T>) -> Polynomial<T> {
		let mut ret = rhs;
		ret += self;
		ret
	}
}

impl<T> Add<&Polynomial<T>> for &Polynomial<T>
where
	T: for<'r> AddAssign<&'r T> + Zero + Clone,
{
	type Output = Polynomial<T>;

	fn add(self, rhs: &Polynomial<T>) -> Polynomial<T> {
		let mut ret = self.clone();
		ret += rhs;
		ret
	}
}

impl<T> AddAssign<Polynomial<T>> for Polynomial<T>
where
	T: for<'r> AddAssign<&'r T> + Zero,
{
	fn add_assign(&mut self, rhs: Polynomial<T>) {
		self.add_assign(&rhs);
	}
}

impl<T> AddAssign<&Polynomial<T>> for Polynomial<T>
where
	T: for<'r> AddAssign<&'r T> + Zero,
{
	fn add_assign(&mut self, rhs: &Polynomial<T>) {
		if self.rev_coeffs.len() < rhs.rev_coeffs.len() {
			// the following line causes clippy to emit a spurious warning (using - in an AddAssign-implementation)
			#[allow(clippy::suspicious_op_assign_impl)]
			let num_add = rhs.rev_coeffs.len() - self.rev_coeffs.len();
			self.rev_coeffs.reserve(num_add);
			for _ in 0..num_add {
				self.rev_coeffs.push(T::zero());
			}
			//self.rev_coeffs.resize_with(rhs.rev_coeffs.len(), T::zero); // SmallVec doesn't have resize_with :(
		}
		self.rev_coeffs
			.iter_mut()
			.zip(rhs.rev_coeffs.iter())
			.for_each(|(l, r)| *l += r);
		self.fixup_coefficients();
	}
}

impl<T> Zero for Polynomial<T>
where
	T: for<'r> AddAssign<&'r T> + Zero,
{
	fn zero() -> Self {
		Self {
			rev_coeffs: smallvec![T::zero()],
		}
	}

	fn is_zero(&self) -> bool {
		self.order() == 0 && self.rev_coeffs.first().unwrap().is_zero()
	}
}

#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn test_add() {
		let a = Polynomial::new(coefficients![1f32, 3.0, 3.0, 0.0]);
		let b = Polynomial::new(coefficients![1f32, 0.0, 1.0]);

		let expected = coefficients![1f32, 4.0, 3.0, 1.0];

		// Polynomial + Polynomial
		let c = a.clone() + b.clone();
		assert_eq!(c.order(), 3);
		assert_eq!(c.coeffs(), expected);

		// Polynomial + Polynomial (swapped)
		let c = b.clone() + a.clone();
		assert_eq!(c.order(), 3);
		assert_eq!(c.coeffs(), expected);

		// Polynomial + &Polynomial
		let c = a.clone() + &b;
		assert_eq!(c.order(), 3);
		assert_eq!(c.coeffs(), expected);

		// &Polynomial + Polynomial
		let c = &a + b.clone();
		assert_eq!(c.order(), 3);
		assert_eq!(c.coeffs(), expected);

		// &Polynomial + &Polynomial
		let c = &a + &b;
		assert_eq!(c.order(), 3);
		assert_eq!(c.coeffs(), expected);

		// Removal of leading zeros
		let b = Polynomial::new(coefficients![-1f32, -3.0, -3.0, 1.0]);
		let c = a + b;
		assert_eq!(c.order(), 0);
		assert_eq!(c.coeffs(), coefficients![1f32]);
	}

	#[test]
	fn test_add_assign() {
		let a = Polynomial::new(coefficients![1f32, 3.0, 3.0, 0.0]);
		let b = Polynomial::new(coefficients![1f32, 0.0, 1.0]);

		let expected = coefficients![1f32, 4.0, 3.0, 1.0];

		// Polynomial += Polynomial
		let mut c = a.clone();
		c += b.clone();
		assert_eq!(c.order(), 3);
		assert_eq!(c.coeffs(), expected);

		// Polynomial += &Polynomial
		let mut c = a.clone();
		c += &b;
		assert_eq!(c.order(), 3);
		assert_eq!(c.coeffs(), expected);

		// Polynomial += Polynomial (swapped)
		let mut c = b.clone();
		c += a.clone();
		assert_eq!(c.order(), 3);
		assert_eq!(c.coeffs(), expected);

		// Polynomial += &Polynomial (swapped)
		let mut c = b.clone();
		c += &a;
		assert_eq!(c.order(), 3);
		assert_eq!(c.coeffs(), expected);

		// Removal of leading zeros
		let b = Polynomial::new(coefficients![-1f32, -3.0, -3.0, 1.0]);
		let mut c = a.clone();
		c += b;
		assert_eq!(c.order(), 0);
		assert_eq!(c.coeffs(), coefficients![1f32]);
	}
}
