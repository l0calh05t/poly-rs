# Poly – Generic dense polynomials

This crate implements type-generic dense polynomial arithmetic.

## Usage

The following code computes and prints the result of the division with remainder of two polynomials in single-precision floating point:

```rust
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
```

Resulting in:

> ```text
> (ω³ + 2ω² + 3ω) / (ω² + 1) = (ω² + 1) * (ω + 2) + 2ω + -2
> ```

Additionally, `poly` allows for evaluation of polynomials with mixed types and the evaluation of the *n*-th derivative of polynomials:

```rust
let x = Complex::new(0f32, 1.0);
let e = a.eval(x);
println!("{} = {} for x = {}", a.to_display("x"), e, x);

let d = a.eval_der(1f32, 2);
println!("({})'' = {} for z = {}", a.to_display("z"), d, 1f32);
```

Resulting in:

> ```text
> x³ + 2x² + 3x = -2+2i for x = 0+1i
> (z³ + 2z² + 3z)'' = 10 for z = 1
> ```

## Status

This is currently an early prototype and the API is likely to change.
Additionally, it is not tuned for performance beyond using [`SmallVec`](https://github.com/servo/rust-smallvec) for coefficient storage.
Left-scalar multiplication (scalar · polynomial) is not implemented generically, but for a fixed list of types due to Rust generic trait implementation restrictions.
Hints on how to improve this crate are welcome.
Aside from that, the main missing piece is currently an implementation of a root-finding algorithm (probably [Bairstow's method](https://en.wikipedia.org/wiki/Bairstow%27s_method), as my focus is on polynomials with real-valued coefficients).
