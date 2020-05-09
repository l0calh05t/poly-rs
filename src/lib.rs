use core::convert::TryFrom;
use core::ops::{AddAssign, Div, Mul, MulAssign, SubAssign};
use num::traits::{MulAddAssign, Pow};
use num::Zero;
use smallvec::SmallVec;

mod add;
mod display;
mod mul;
mod sub;
pub use add::*;
pub use display::*;
pub use mul::*;
pub use sub::*;

#[cfg(debug_assertions)]
macro_rules! assert_assume {
	($cond:expr) => {
		assert!($cond);
	};
	($cond:expr, $($arg:tt)+) => {
		assert!($cond, $($arg)+);
	};
}

#[cfg(not(debug_assertions))]
macro_rules! assert_assume {
	($cond:expr) => {
		if !($cond) {
			unsafe {
				std::hint::unreachable_unchecked();
				}
			}
	};
	($cond:expr, $($arg:tt)+) => {
		if !($cond) {
			unsafe {
				std::hint::unreachable_unchecked();
				}
			}
	};
}

#[macro_export]
macro_rules! coefficients {
	($elem:expr; $n:expr) => ({
		use smallvec::{smallvec, SmallVec};
		let ret : SmallVec<[_; 8]> = smallvec![$elem; $n];
		ret
	});
	($($x:expr),*$(,)*) => ({
		use smallvec::{smallvec, SmallVec};
		let ret : SmallVec<[_; 8]> = smallvec![$($x,)*];
		ret
	});
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Polynomial<T> {
	rev_coeffs: SmallVec<[T; 8]>,
}

impl<T> Polynomial<T> {
	pub fn order(&self) -> i32 {
		assert_assume!(!self.rev_coeffs.is_empty());
		assert_assume!(i32::try_from(self.rev_coeffs.len() - 1).is_ok());
		(self.rev_coeffs.len() - 1) as i32
	}

	pub fn reverse_coeffs(&self) -> &SmallVec<[T; 8]> {
		&self.rev_coeffs
	}

	pub fn into_coeffs(self) -> SmallVec<[T; 8]> {
		let mut coeffs = self.rev_coeffs;
		coeffs.reverse();
		coeffs
	}

	pub fn into_reverse_coeffs(self) -> SmallVec<[T; 8]> {
		self.rev_coeffs
	}

	pub fn coeffs(&self) -> SmallVec<[T; 8]>
	where
		T: Clone,
	{
		let mut coeffs = self.rev_coeffs.clone();
		coeffs.reverse();
		coeffs
	}

	fn fixup_coefficients(&mut self)
	where
		T: Zero,
	{
		while {
			if let Some(x) = self.rev_coeffs.last() {
				x.is_zero()
			} else {
				false
			}
		} {
			self.rev_coeffs.pop();
		}
		if self.rev_coeffs.is_empty() {
			self.rev_coeffs.push(T::zero());
		}
		assert!(i32::try_from(self.rev_coeffs.len() - 1).is_ok());
	}

	pub fn new(coefficients: SmallVec<[T; 8]>) -> Self
	where
		T: Zero,
	{
		let mut rev_coeffs = coefficients;
		rev_coeffs.reverse();
		Self::new_reversed(rev_coeffs)
	}

	pub fn new_reversed(rev_coeffs: SmallVec<[T; 8]>) -> Self
	where
		T: Zero,
	{
		let mut ret = Self { rev_coeffs };
		ret.fixup_coefficients();
		ret
	}

	pub fn eval<X, Y>(&self, x: X) -> Y
	where
		T: Zero,
		for<'l, 'r> &'l T: Mul<&'r X, Output = Y>,
		X: for<'r> MulAssign<&'r X> + num::One,
		Y: AddAssign + Zero,
	{
		let mut xn = X::one();
		let mut y = Y::zero();
		for a in self.rev_coeffs.iter() {
			y += a * &xn;
			xn *= &x;
		}
		y
	}

	pub fn eval_precise<X, Y>(&self, x: X) -> Y
	where
		T: Zero + Clone + Into<Y>,
		for<'l> &'l X: Pow<i32, Output = X>,
		Y: MulAddAssign<X, Y> + Zero,
		// this trait is only required to guide type inference
		for<'l> &'l T: Mul<X, Output = Y>,
	{
		let mut y = Y::zero();
		for (a, e) in self.rev_coeffs.iter().zip(0i32..) {
			// in num, MulAddAssign is only defined for values not references so we clone a
			let mut ay = a.clone().into();
			let mut y_old = Y::zero();
			core::mem::swap(&mut y, &mut y_old);
			ay.mul_add_assign(x.pow(e), y_old);
			core::mem::swap(&mut y, &mut ay);
		}
		y
	}

	pub fn eval_der<X, Y>(&self, x: X, n: i32) -> Y
	where
		T: Zero + num::FromPrimitive + for<'r> Mul<&'r X, Output = Y>,
		for<'l> &'l T: Mul<T, Output = T>,
		X: for<'r> MulAssign<&'r X> + num::One,
		Y: AddAssign + Zero,
	{
		assert!(n > 0);
		let mut xn = X::one();
		let mut y = Y::zero();
		for ((a, e_old), e_new) in self
			.rev_coeffs
			.iter()
			.zip(0i32..)
			.skip(n as usize)
			.zip(0i32..)
		{
			let mul = ((e_new + 1)..=e_old).product();
			y += (a * T::from_i32(mul).unwrap()) * &xn;
			xn *= &x;
		}
		y
	}

	pub fn eval_der_precise<X, Y>(&self, x: X, n: i32) -> Y
	where
		T: Zero + num::FromPrimitive + Into<Y>,
		for<'l> &'l T: Mul<T, Output = T>,
		for<'l> &'l X: Pow<i32, Output = X>,
		Y: MulAddAssign<X, Y> + Zero + Clone,
		// this trait is only required to guide type inference
		for<'l> &'l T: Mul<X, Output = Y>,
	{
		let mut y = Y::zero();
		for ((a, e_old), e_new) in self
			.rev_coeffs
			.iter()
			.zip(0i32..)
			.skip(n as usize)
			.zip(0i32..)
		{
			let mul = ((e_new + 1)..=e_old).product();
			let mut ay = (a * T::from_i32(mul).unwrap()).into();
			let mut y_old = Y::zero();
			core::mem::swap(&mut y, &mut y_old);
			ay.mul_add_assign(x.pow(e_new), y_old);
			core::mem::swap(&mut y, &mut ay);
		}
		y
	}

	pub fn div_rem(&self, rhs: &Self) -> (Self, Self)
	where
		T: Zero + Clone + for<'r> AddAssign<&'r T> + SubAssign,
		for<'l, 'r> &'l T: Mul<&'r T, Output = T> + Div<&'r T, Output = T>,
	{
		assert!(!rhs.is_zero());

		let order_l = self.order() as usize;
		let order_r = rhs.order() as usize;
		if order_l < order_r {
			return (Self::zero(), self.clone());
		}
		let order_o = order_l - order_r;

		let rhs = &rhs.rev_coeffs;
		let mut remainder = self.rev_coeffs.clone();
		let mut quotient = coefficients![T::zero(); order_o + 1];

		for el in (order_r..=order_l).rev() {
			let v = &remainder[el] / &rhs[order_r];
			remainder[el] = T::zero();
			for k in 1..=order_r {
				remainder[el - k] -= &v * &rhs[order_r - k];
			}
			quotient[el - order_r] = v;
		}

		(Self::new_reversed(quotient), Self::new_reversed(remainder))
	}
}

#[cfg(test)]
mod tests {
	use crate::*;

	#[cfg(not(debug_assertions))]
	use smallvec::SmallVec;

	#[test]
	fn test_new() {
		let poly = Polynomial::new(coefficients![0f32, 0.0, 1.0, 2.0, 3.0, 0.0]);
		assert_eq!(poly.order(), 3);
		let mut coeffs = coefficients![1f32, 2.0, 3.0, 0.0];
		assert_eq!(poly.coeffs(), coeffs);
		coeffs.reverse();
		assert_eq!(*poly.reverse_coeffs(), coeffs);
	}

	#[cfg(not(debug_assertions))]
	#[test]
	fn test_new_max_size() {
		let mut coefficients = SmallVec::new();
		coefficients.resize(
			usize::try_from(core::i32::MAX)
				.unwrap()
				.checked_add(1)
				.unwrap(),
			1u8,
		);
		let poly = Polynomial::new_reversed(coefficients);
		assert_eq!(poly.order(), core::i32::MAX);
	}

	#[cfg(not(debug_assertions))]
	#[test]
	#[should_panic(expected = "assertion failed: i32::try_from(self.rev_coeffs.len() - 1).is_ok()")]
	fn test_new_oversized() {
		let mut coefficients = SmallVec::new();
		coefficients.resize(
			usize::try_from(core::i32::MAX)
				.unwrap()
				.checked_add(2)
				.unwrap(),
			1u8,
		);
		Polynomial::new_reversed(coefficients);
	}

	#[test]
	fn test_eval() {
		let poly = Polynomial::new(coefficients![1f32, 2.0, 3.0, 0.0]);
		assert_eq!(poly.eval(2f32), 22.0);
		assert_eq!(
			poly.eval(num::Complex::new(0f32, 1.0)),
			num::Complex::new(-2f32, 2.0)
		);
		assert_eq!(poly.eval_precise(2f32), 22.0);
	}

	#[test]
	fn test_eval_substitution() {
		let mut poly = Polynomial::new(coefficients![1f32, 2.0, 3.0, 0.0]);
		poly = poly.eval(Polynomial::new(coefficients![1f32, 0.0, 0.0])); // x → x²
		assert_eq!(poly.order(), 6);
		assert_eq!(
			poly.into_coeffs(),
			coefficients![1f32, 0.0, 2.0, 0.0, 3.0, 0.0, 0.0]
		);
		poly = Polynomial::new(coefficients![1f32, 2.0, 3.0, 0.0]);
		poly = poly.eval(Polynomial::new(coefficients![1f32, 0.0, 1.0])); // x → x² + 1
		assert_eq!(
			poly.into_coeffs(),
			coefficients![1f32, 0.0, 5.0, 0.0, 10.0, 0.0, 6.0]
		);
	}

	#[test]
	fn test_eval_der() {
		// x³ + 2x² + 3x + 0
		let poly = Polynomial::new(coefficients![1f32, 2.0, 3.0, 0.0]);
		// 3x² + 4x + 3
		assert_eq!(poly.eval_der(0f32, 1), 3.0);
		assert_eq!(poly.eval_der(1f32, 1), 10.0);
		assert_eq!(poly.eval_der(2f32, 1), 23.0);
		assert_eq!(poly.eval_der_precise(0f32, 1), 3.0);
		assert_eq!(poly.eval_der_precise(1f32, 1), 10.0);
		assert_eq!(poly.eval_der_precise(2f32, 1), 23.0);
		// 6x + 4
		assert_eq!(poly.eval_der(0f32, 2), 4.0);
		assert_eq!(poly.eval_der(1f32, 2), 10.0);
		assert_eq!(poly.eval_der_precise(0f32, 2), 4.0);
		assert_eq!(poly.eval_der_precise(1f32, 2), 10.0);
	}

	#[test]
	fn test_div_rem() {
		let poly_a = Polynomial::new(coefficients![1f32, 2.0, 3.0, 0.0]);

		// (x³ + 2x² + 3x) / x = x² + 2x + 3 | 0
		let poly_b = Polynomial::new(coefficients![1f32, 0.0]);
		let (q, r) = poly_a.div_rem(&poly_b);
		assert_eq!(q.coeffs(), coefficients![1f32, 2.0, 3.0]);
		assert!(r.is_zero());

		// (x³ + 2x² + 3x) / 2x = 0.5x² + x + 1.5 | 0
		let poly_b = Polynomial::new(coefficients![2f32, 0.0]);
		let (q, r) = poly_a.div_rem(&poly_b);
		assert_eq!(q.coeffs(), coefficients![0.5f32, 1.0, 1.5]);
		assert!(r.is_zero());

		// (x³ + 2x² + 3x) / x² = x + 2 | 3x
		let poly_b = Polynomial::new(coefficients![1f32, 0.0, 0.0]);
		let (q, r) = poly_a.div_rem(&poly_b);
		assert_eq!(q.coeffs(), coefficients![1f32, 2.0]);
		assert_eq!(r.coeffs(), coefficients![3f32, 0.0]);

		// (x³ + 2x² + 3x) / (x² + 1) = x + 2 | 2x - 2
		let poly_b = Polynomial::new(coefficients![1f32, 0.0, 1.0]);
		let (q, r) = poly_a.div_rem(&poly_b);
		assert_eq!(q.coeffs(), coefficients![1f32, 2.0]);
		assert_eq!(r.coeffs(), coefficients![2f32, -2.0]);

		// (x² + 1) / (x³ + 2x² + 3x) = 0 | x² + 1
		let (q, r) = poly_b.div_rem(&poly_a);
		assert!(q.is_zero());
		assert_eq!(r, poly_b);
	}

	#[test]
	#[should_panic(expected = "assertion failed: !rhs.is_zero()")]
	fn test_div_rem_zero() {
		let poly_a = Polynomial::new(coefficients![1f32, 2.0, 3.0, 0.0]);
		let poly_b = Polynomial::zero();
		let _ = poly_a.div_rem(&poly_b);
	}
}
