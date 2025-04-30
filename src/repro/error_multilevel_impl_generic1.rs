trait A {}
trait AT: A {}

struct S1;
struct S2;

impl A for S1 {}
impl A for S2 {}

impl AT for S1 {}
impl AT for S2 {}

trait F<X> {
    fn make(x: u32) -> Self where Self: Sized;
}

impl F<S1> for S1 {
    fn make(x: u32) -> Self { S1 }
}

impl F<S2> for S2 {
    fn make(x: u32) -> Self { S2 }
}

trait C<X> {}

impl<X: AT, T: F<X>> C<X> for T {}
impl<X: AT, T1: F<X>, T2: F<X>> C<X> for (T1, T2) {} // <-- Error
/*
error[E0119]: conflicting implementations of trait `C<_>` for type `(_, _)`
  --> src/repro/error_multilevel_impl_generic1.rs:28:1
   |
27 | impl<X: AT, T: F<X>> C<X> for T {}
   | ------------------------------- first implementation here
28 | impl<X: AT, T1: F<X>, T2: F<X>> C<X> for (T1, T2) {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(_, _)`
   |
   = note: downstream crates may implement trait `F<_>` for type `(_, _)`
 */

 /*
  We can comment out `impl<X: AT, T: F<X>> C<X> for T {}` and also be OK. The issue seems to be that it is
  possible for a (F<X>, F<X>) to itself be an F<X>, leading to conflict.
  Below is a remedy, however it comes with a heavy cost of no longer being able to work with linearized n-arity tuples
  of `T, (T, T), (T, T, T), ...`, instead we will have to pay a probably unjustifiable cost of exponential scaling by 
  n in S{n}, or for full possibility space of the n-arity linearization map, factorial scaling.
  */

// impl<T1: F<S1>, T2: F<S1>> C<S1> for (T1, T2) {}
// impl<T1: F<S2>, T2: F<S2>> C<S2> for (T1, T2) {}

/*
Alternatively, We can use (T,) which cannot be a (T1, T2). But T CAN be (T1, T2). This keeps linearization per arity
 */

// impl<X: AT, T: F<X>> C<X> for (T,) {}
// impl<X: AT, T1: F<X>, T2: F<X>> C<X> for (T1, T2) {}