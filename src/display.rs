use crate::Polynomial;

use num::{One, Zero};
use std::fmt::{Display, Error, Formatter};

pub struct DisplayPolynomial<'a, 'b, T: Display + One + Zero + PartialEq> {
	variable: &'a str,
	polynomial: &'b Polynomial<T>,
}

impl<T: Display + One + Zero + PartialEq> Polynomial<T> {
	pub fn to_display<'a, 'b>(&'b self, variable: &'a str) -> DisplayPolynomial<'a, 'b, T> {
		DisplayPolynomial {
			variable,
			polynomial: self,
		}
	}
}

fn to_superscript(k: &str) -> Result<String, Error> {
	k.chars()
		.map(|x| match x {
			'0' => Ok('⁰'),
			'1' => Ok('¹'),
			'2' => Ok('²'),
			'3' => Ok('³'),
			'4' => Ok('⁴'),
			'5' => Ok('⁵'),
			'6' => Ok('⁶'),
			'7' => Ok('⁷'),
			'8' => Ok('⁸'),
			'9' => Ok('⁹'),
			_ => Err(Error),
		})
		.collect()
}

impl<'a, 'b, T: Display + One + Zero + PartialEq> Display for DisplayPolynomial<'a, 'b, T> {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		let mut first = true;
		for (k, v) in self.polynomial.rev_coeffs.iter().enumerate().rev() {
			if v.is_zero() {
				continue;
			}
			if !first {
				write!(f, " + ")?;
			}
			first = false;
			if k == 0 || !v.is_one() {
				write!(f, "{}", v)?;
			}
			if k > 0 {
				write!(f, "{}", self.variable)?;
			}
			if k > 1 {
				let k = to_superscript(&k.to_string())?;
				write!(f, "{}", k)?;
			}
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::*;
	use smallvec::SmallVec;

	#[test]
	fn test_display() {
		let poly = Polynomial::new(coefficients![1, 3, 2, 0, 1, 0]);
		let poly_string = poly.to_display("x").to_string();
		assert_eq!(poly_string, "x⁵ + 3x⁴ + 2x³ + x");

		let mut coeffs = SmallVec::new();
		coeffs.resize(11, 0);
		*coeffs.first_mut().unwrap() = 1;
		*coeffs.last_mut().unwrap() = 1;
		let poly = Polynomial::new(coeffs);
		let poly_string = poly.to_display("ω").to_string();
		assert_eq!(poly_string, "ω¹⁰ + 1");
	}
}
