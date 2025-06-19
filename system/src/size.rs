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

/// Represents width and height as `u32`.
///
/// The W3C DOM API makes heavy use of floating point, but constraining values
/// to be integers helps avoid subtle issues, such as rendering artifacts on
/// canvas elements. This struct therefore represents numbers as `u32`, which
/// (unlike `usize`) is `Into<f64>`.
#[derive(Clone, Copy, Default)]
pub struct Size {
    pub height: u32,
    pub width: u32,
}
