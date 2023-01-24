use core::convert::Infallible;

pub trait UnwrapOptimized {
    type Output;
    fn unwrap_optimized(self) -> Self::Output;
}

impl<T> UnwrapOptimized for Option<T> {
    type Output = T;

    #[inline(always)]
    fn unwrap_optimized(self) -> Self::Output {
        #[cfg(target_family = "wasm")]
        match self {
            Some(t) => t,
            None => core::arch::wasm32::unreachable(),
        }
        #[cfg(not(target_family = "wasm"))]
        self.unwrap()
    }
}

impl<T, E: core::fmt::Debug> UnwrapOptimized for Result<T, E> {
    type Output = T;

    #[inline(always)]
    fn unwrap_optimized(self) -> Self::Output {
        #[cfg(target_family = "wasm")]
        match self {
            Ok(t) => t,
            Err(_) => core::arch::wasm32::unreachable(),
        }
        #[cfg(not(target_family = "wasm"))]
        self.unwrap()
    }
}

pub trait UnwrapInfallible {
    type Output;
    fn unwrap_infallible(self) -> Self::Output;
}

impl<T> UnwrapInfallible for Result<T, Infallible> {
    type Output = T;

    fn unwrap_infallible(self) -> Self::Output {
        match self {
            Ok(ok) => ok,
            // In the following `Err(never)` branch we convert a type from
            // `Infallible` to `!`. Both of these are empty types and are
            // essentially synonyms in rust, they differ only due to historical
            // reasons that will eventually be eliminated. `Infallible` is a
            // version we can put in a structure, and `!` is one that gets some
            // special control-flow treatments.
            //
            // Specifically: the type `!` of the resulting expression will be
            // considered an acceptable inhabitant of any type -- including
            // `Self::Output` -- since it's an impossible path to execute, this
            // is considered a harmless convenience in the type system, a bit
            // like defining zero-divided-by-anything as zero.
            //
            // We could also write an infinite `loop {}` here or
            // `unreachable!()` or similar expressions of type `!`, but
            // destructuring the `never` variable into an empty set of cases is
            // the most honest since it's statically checked to _be_ infallible,
            // not just an assertion of our hopes.)
            Err(never) => match never {},
        }
    }
}
