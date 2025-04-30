struct SA1;
struct SA2;

struct SB1;
struct SB2;

trait A {}
trait B {}

impl A for SA1 {}
impl A for SA2 {}

impl B for SB1 {}
impl B for SB2 {}

trait F<X> { fn make() -> Self where Self: Sized; }

impl F<SA1> for SA1 { fn make() -> Self { SA1 } }
impl F<SA2> for SA2 { fn make() -> Self { SA2 } }
impl F<SB1> for SB1 { fn make() -> Self { SB1 } }
impl F<SB2> for SB2 { fn make() -> Self { SB2 } }

trait C<X> {}

impl<X: A, T1: F<X>> C<X> for (T1,) {}
impl<X: A, T1: F<X>, T2: F<X>> C<X> for (T1, T2) {}
impl<X: B, T1: F<X>> C<X> for (T1,) {} // <-- Error
/*
error[E0119]: conflicting implementations of trait `C<_>` for type `(_,)`
  --> src/repro/error_multilevel_impl_generic2.rs:27:1
   |
25 | impl<X: A, T1: F<X>> C<X> for (T1,) {}
   | ----------------------------------- first implementation here
26 | impl<X: A, T1: F<X>, T2: F<X>> C<X> for (T1, T2) {}
27 | impl<X: B, T1: F<X>> C<X> for (T1,) {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(_,)`

 */