#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! # or
//!
//! A generalized Result.
//!

/// A generalized Result, just a two-variant enum.
///
/// Much of the functionality of Result and Option is not redundantly
/// provided here. An Option is always just a few characters away through
/// the `a` and `b` methods, which you should combine with `as_ref` and
/// `as_mut` to get the full spectrum of provided functionality.
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum Or<A, B> {
    /// One variant
    A(A),
    /// Another variant
    B(B)
}

impl<A, B> Or<A, B> {
    /// Returns true if the `Or` is `A`
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use or::Or;
    /// let x: Or<i32, ()> = Or::A(545);
    /// assert!(x.is_a());
    ///
    /// let y: Or<(), i32> = Or::B(2);
    /// assert!(!y.is_a());
    /// ```
    pub fn is_a(&self) -> bool {
        self.as_ref().a().is_some()
    }

    /// Returns true if the `Or` is `B`
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use or::Or;
    /// let x: Or<i32, ()> = Or::A(545);
    /// assert!(!x.is_b());
    ///
    /// let y: Or<(), i32> = Or::B(2);
    /// assert!(y.is_b());
    /// ```
    pub fn is_b(&self) -> bool {
        self.as_ref().b().is_some()
    }

    /// Converts form `Or<A, B>` to `Option<A>`
    ///
    /// This method consumes `self` and discards `B`, if any.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use or::Or;
    /// let x: Or<i32, ()> = Or::A(545);
    /// assert_eq!(x.a(), Some(545));
    ///
    /// let y: Or<(), i32> = Or::B(2);
    /// assert!(y.a().is_none());
    /// ```
    pub fn a(self) -> Option<A> {
        match self {
            Or::A(a) => Some(a),
            _ => None
        }
    }

    /// Converts form `Or<A, B>` to `Option<B>`
    ///
    /// This method consumes `self` and discards `A`, if any.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use or::Or;
    /// let x: Or<i32, ()> = Or::A(545);
    /// assert!(x.b().is_none());
    ///
    /// let y: Or<(), i32> = Or::B(2);
    /// assert_eq!(y.b(), Some(2));
    /// ```
    pub fn b(self) -> Option<B> {
        match self {
            Or::B(b) => Some(b),
            _ => None
        }
    }

    /// Convert from `&Or<A, B>` to `Or<&A, &B>`
    ///
    /// The returned `Or` contains references into the existing
    /// `Or`, which is left in place.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use or::Or;
    /// let x: Or<String, ()> = Or::A("hello".to_string());
    /// assert_eq!(&**x.as_ref().a().unwrap(), "hello");
    ///
    /// let y: Or<(), i32> = Or::B(2);
    /// assert!(y.as_ref().a().is_none());
    /// ```
    pub fn as_ref(&self) -> Or<&A, &B> {
        match *self {
            Or::A(ref a) => Or::A(a),
            Or::B(ref b) => Or::B(b),
        }
    }

    /// Convert from `&mut Or<A, B>` to `Or<&mut A, &mut B>`
    ///
    /// The returned `Or` contains references into the existing
    /// `Or`, which is left in place.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use or::Or;
    /// let mut x: Or<String, ()> = Or::A("hello".to_string());
    /// x.as_mut().a().map(|s| s.push_str(" world!"));
    ///
    /// assert_eq!(&**x.as_ref().a().unwrap(), "hello world!");
    /// ```
    pub fn as_mut(&mut self) -> Or<&mut A, &mut B> {
        match *self {
            Or::A(ref mut a) => Or::A(a),
            Or::B(ref mut b) => Or::B(b),
        }
    }

    /// Convert from `Or<A, B>` to `Or<B, A>`
    ///
    /// Consumes `self` and returns a new `Or`
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use or::Or;
    /// let x: Or<(), i32> = Or::A(73).swap();
    ///
    /// assert_eq!(x.b().unwrap(), 73);
    /// ```
    pub fn swap(self) -> Or<B, A> {
        match self {
            Or::A(b) => Or::B(b),
            Or::B(a) => Or::A(a)
        }
    }
}

