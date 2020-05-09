use num::Complex;
use poly::{coefficients, Polynomial};

fn main() {
	let a = Polynomial::new(coefficients![1f32, 2.0, 3.0, 0.0]);
	let b = Polynomial::new(coefficients![1f32, 0.0, 1.0]);
	let (q, r) = a.div_rem(&b);
	println!(
		"({0}) / ({1}) = ({1}) * ({2}) + {3}",
		a.to_display("ω"),
		b.to_display("ω"),
		q.to_display("ω"),
		r.to_display("ω")
	);

	let x = Complex::new(0f32, 1.0);
	let e = a.eval(x);
	println!("{} = {} for x = {}", a.to_display("x"), e, x);

	let d = a.eval_der(1f32, 2);
	println!("({})'' = {} for z = {}", a.to_display("z"), d, 1f32);
}
