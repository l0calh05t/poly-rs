use crate::Polynomial;

use core::ops::{Neg, Sub, SubAssign};
use num::Zero;

impl<T> Sub<Polynomial<T>> for Polynomial<T>
where
	T: for<'r> SubAssign<&'r T> + Zero,
{
	type Output = Polynomial<T>;

	fn sub(self, rhs: Polynomial<T>) -> Polynomial<T> {
		let mut ret = self;
		ret -= &rhs;
		ret
	}
}

impl<T> Sub<&Polynomial<T>> for Polynomial<T>
where
	T: for<'r> SubAssign<&'r T> + Zero,
{
	type Output = Polynomial<T>;

	fn sub(self, rhs: &Polynomial<T>) -> Polynomial<T> {
		let mut ret = self;
		ret -= rhs;
		ret
	}
}

impl<T> Sub<Polynomial<T>> for &Polynomial<T>
where
	T: for<'r> SubAssign<&'r T> + Zero + Clone,
{
	type Output = Polynomial<T>;

	fn sub(self, rhs: Polynomial<T>) -> Polynomial<T> {
		let mut ret = self.clone();
		ret -= &rhs;
		ret
	}
}

impl<T> Sub<&Polynomial<T>> for &Polynomial<T>
where
	T: for<'r> SubAssign<&'r T> + Zero + Clone,
{
	type Output = Polynomial<T>;

	fn sub(self, rhs: &Polynomial<T>) -> Polynomial<T> {
		let mut ret = self.clone();
		ret -= rhs;
		ret
	}
}

impl<T> SubAssign<Polynomial<T>> for Polynomial<T>
where
	T: for<'r> SubAssign<&'r T> + Zero,
{
	fn sub_assign(&mut self, rhs: Polynomial<T>) {
		self.sub_assign(&rhs);
	}
}

impl<T> SubAssign<&Polynomial<T>> for Polynomial<T>
where
	T: for<'r> SubAssign<&'r T> + Zero,
{
	fn sub_assign(&mut self, rhs: &Polynomial<T>) {
		if self.rev_coeffs.len() < rhs.rev_coeffs.len() {
			// the following line causes clippy to emit a spurious warning (using - in an SubAssign-implementation)
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
			.for_each(|(l, r)| *l -= r);
		self.fixup_coefficients();
	}
}

impl<T> Neg for Polynomial<T>
where
	T: Zero,
	for<'l> &'l T: Neg<Output = T>,
{
	type Output = Polynomial<T>;

	fn neg(self) -> Polynomial<T> {
		let mut rev_coeffs = self.rev_coeffs;
		for c in rev_coeffs.iter_mut() {
			*c = -&*c;
		}
		Polynomial::new_reversed(rev_coeffs)
	}
}

impl<T> Neg for &Polynomial<T>
where
	T: Zero,
	for<'l> &'l T: Neg<Output = T>,
{
	type Output = Polynomial<T>;

	fn neg(self) -> Polynomial<T> {
		let rev_coeffs = self.rev_coeffs.iter().map(|c| -c).collect();
		Polynomial::new_reversed(rev_coeffs)
	}
}

#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn test_sub() {
		let a = Polynomial::new(coefficients![1f32, 3.0, 3.0, 0.0]);
		let b = Polynomial::new(coefficients![1f32, 0.0, 1.0]);

		let expected = coefficients![1f32, 2.0, 3.0, -1.0];

		// Polynomial - Polynomial
		let c = a.clone() - b.clone();
		assert_eq!(c.order(), 3);
		assert_eq!(c.coeffs(), expected);

		// Polynomial - &Polynomial
		let c = a.clone() - &b;
		assert_eq!(c.order(), 3);
		assert_eq!(c.coeffs(), expected);

		// &Polynomial - Polynomial
		let c = &a - b.clone();
		assert_eq!(c.order(), 3);
		assert_eq!(c.coeffs(), expected);

		// &Polynomial - &Polynomial
		let c = &a - &b;
		assert_eq!(c.order(), 3);
		assert_eq!(c.coeffs(), expected);

		// Removal of leading zeros
		let b = Polynomial::new(coefficients![1f32, 3.0, 3.0, -1.0]);
		let c = a - b;
		assert_eq!(c.order(), 0);
		assert_eq!(c.coeffs(), coefficients![1f32]);
	}

	#[test]
	fn test_sub_assign() {
		let a = Polynomial::new(coefficients![1f32, 3.0, 3.0, 0.0]);
		let b = Polynomial::new(coefficients![1f32, 0.0, 1.0]);

		let expected = coefficients![1f32, 2.0, 3.0, -1.0];

		// Polynomial -= Polynomial
		let mut c = a.clone();
		c -= b.clone();
		assert_eq!(c.order(), 3);
		assert_eq!(c.coeffs(), expected);

		// Polynomial -= &Polynomial
		let mut c = a.clone();
		c -= &b;
		assert_eq!(c.order(), 3);
		assert_eq!(c.coeffs(), expected);

		// Removal of leading zeros
		let b = Polynomial::new(coefficients![1f32, 3.0, 3.0, -1.0]);
		let mut c = a.clone();
		c -= b;
		assert_eq!(c.order(), 0);
		assert_eq!(c.coeffs(), coefficients![1f32]);
	}

	#[test]
	fn test_neg() {
		let a = Polynomial::new(coefficients![1f32, 3.0, 3.0, 0.0]);
		let expected = coefficients![-1f32, -3.0, -3.0, 0.0];

		// &Polynomial
		let b = -&a;
		assert_eq!(b.coeffs(), expected);

		// -Polynomial
		let b = -a;
		assert_eq!(b.coeffs(), expected);
	}
}
