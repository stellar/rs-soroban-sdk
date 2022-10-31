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
