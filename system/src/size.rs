//! Types and conversions to ease web client interaction.

#[must_use]
pub const fn u32_to_usize(value: u32) -> usize {
    const { assert!(size_of::<u32>() <= size_of::<usize>()) }
    value as usize
}

/// Infallible, because Wasm `usize` is guaranteed to be 32 bits.
#[cfg(target_arch = "wasm32")]
#[must_use]
pub const fn usize_to_u32(value: usize) -> u32 {
    const { assert!(size_of::<usize>() <= size_of::<u32>()) }
    value as u32
}

#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub const fn f64_to_u32_saturating(value: f64) -> u32 {
    // The lint allowances are required because of this `as` cast, but it (is
    // the only construct that) does exactly what we want here. See also:
    // <https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.as.numeric.float-as-int>
    value as u32
}

/// # Panics
///
/// Will panic on overflow.
#[cfg(not(target_arch = "wasm32"))]
#[expect(clippy::cast_possible_truncation)]
#[must_use]
pub const fn usize_to_u32(value: usize) -> u32 {
    let output = value as u32;
    assert!(output as usize == value, "overflow");
    output
}

/// Represents width and height as `f64`.
#[derive(Clone, Copy, Default)]
pub struct SizeF64 {
    pub height: f64,
    pub width: f64,
}

/// Represents width and height as `u32`.
///
/// The W3C DOM API makes heavy use of floating point, but constraining values
/// to be integers helps avoid subtle issues, such as rendering artifacts on
/// canvas elements. This struct therefore represents numbers as `u32`, which
/// (unlike `usize`) is `Into<f64>`.
#[derive(Clone, Copy, Default)]
pub struct SizeU32 {
    pub height: u32,
    pub width: u32,
}
