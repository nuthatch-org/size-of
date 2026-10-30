#![cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]

#[cfg(target_arch = "wasm32")]
use core::arch::wasm32::v128;
#[cfg(target_arch = "wasm64")]
use core::arch::wasm64::v128;

impl_total_size_childless! {
    v128,
}
