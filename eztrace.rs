//! Zero-fuss debug tracing macro.
//!
//! Cargo.toml:
//! ```text
//! [dependencies]
//! eztrace = "*"
//! ```
//!
//! Usage:
//! ```
//! #[allow(unused_imports)] #[macro_use] extern crate eztrace;
//! # fn main() {
//! # let (my_variable, other_variable) = (42, 237);
//! trace!(my_variable, other_variable);
//! # }
//! ```
//!
//! Prints this:
//! ```text
//! my_variable, other_variable: 42 237
//! ```

/// Prints out variables and their debug representation.
///
/// Non-`Copy` types do what you would hope.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # #[macro_use] extern crate eztrace;
/// let a = 3;
/// let b = 4;
/// let c = 5;
/// trace!(a, b, c);
/// // a, b, c: 3 4 5
/// trace!(a * a + b * b, c * c);
/// // a * a + b * b, c * c: 25 25
/// ```
///
/// To print just the file & line:
///
/// ```
/// # #[macro_use] extern crate eztrace;
/// trace!();
/// // eztrace.rs:1
/// ```
///
/// To print a label:
///
/// ```
/// # #[macro_use] extern crate eztrace;
/// trace!("searching haystack for needles");
/// // searching haystack for needles
/// ```
///
/// You can also prefix with a `#` to get `{:#?}`-style format codes (tho you might just use
/// `dbg!()` instead…)
///
/// ```
/// # #[macro_use] extern crate eztrace;
/// trace!(#);
/// // eztrace.rs:1
/// trace!(#"hello");
/// // "hello"
/// #[derive(Debug, Default)]
/// struct Coords {
///     x: f32,
///     y: f32,
/// }
/// let zero = Coords::default();
/// trace!(#zero);
/// // zero: Coords {
/// //     x: 0.0,
/// //     y: 0.0,
/// // }
/// ```
#[macro_export]
macro_rules! trace {
    () => { println!(trace!(@line)); };
    (#) => { println!(trace!(@line)); };
    (#$label:literal) => {
        println!("{:?}", $label);
    };
    ($label:literal) => {
        println!("{}", $label);
    };
    (#$($IT:expr),* $(,)?) => {
        println!(
            trace!(@#fmt $($IT),*),
            $(&$IT),*
        );
    };
    ($($IT:expr),* $(,)?) => {
        println!(
            trace!(@fmt $($IT),*),
            $(&$IT),*
        );
    };
    (@line) => {
        concat!(
            file!(), ":", line!(),
        )
    };
    (@#fmt $($IT:expr),*) => {
        concat!(
            trace!(@stringify $($IT,)*),
            ":",
            $(trace!(@#fmtcode $IT)),*
        )
    };
    (@fmt $($IT:expr),*) => {
        concat!(
            trace!(@stringify $($IT,)*),
            ":",
            $(trace!(@fmtcode $IT)),*
        )
    };
    (@#fmtcode $_:expr) => {
        " {:#?}"
    };
    (@fmtcode $_:expr) => {
        " {:?}"
    };
    (@stringify $HEAD:expr, $($IT:expr,)*) => {
        concat!(
            stringify!($HEAD),
            $(
                ", ",
                stringify!($IT),
            )*
        )
    };
}


#[cfg(test)]
mod tests {
    #[test]
    fn no_move() {
        let string = format!("hey");
        trace!(string, 9);
        trace!(string, string);
        trace!(
            string,
            string,
            string,
        );
    }

    #[test]
    fn single_eval_per_arg() {
        let mut n = 0;
        fn incr(i: &mut usize) { *i += 1; }
        trace!(
            "check that each argument",
            "is only evaluated",
            "once",
            incr(&mut n),
        );
        trace!(n);
        assert_eq!(n, 1);
    }

    #[test]
    fn empty() {
        trace!();
    }

    #[test]
    fn single() {
        let hello = "hello";
        trace!(hello);
    }

    #[test]
    fn literal() {
        trace!("hi");
        trace!(0xED);
    }

    #[test]
    fn multi() {
        trace!("hello", "world!");
    }

    #[test]
    fn the_docs() {
        let a = 3;
        let b = 4;
        let c = 5;
        trace!(a, b, c);
        // a, b, c: 3 4 5
        trace!(a * a + b * b, c * c);
        // a * a + b * b, c * c: 25 25
        trace!();


        #[derive(Debug, Default)]
        struct Coords {
            x: f32,
            y: f32,
        }
        trace!(#);
        // eztrace.rs:7
        trace!(#"hello");
        // "hello"
        let zero = Coords::default();
        trace!(#zero);
        // zero: Coords {
        //     x: 0.0,
        //     y: 0.0,
        // }
    }
}

// FIXME: Maybe the macro should emit a warning?
// FIXME: Feature to always fail.
