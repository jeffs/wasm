const COLORS: [&str; 14] = [
    "#FF0000", //  2,  47 Red
    "#00FF00", //  3,  53 Lime
    "#0000FF", //  5,  59 Blue
    "#FFFF00", //  7,  61 Yellow
    "#00FFFF", // 11,  67 Cyan
    "#FF00FF", // 13,  71 Magenta
    "#C0C0C0", // 17,  73 Silver
    "#808080", // 19,  79 Gray
    "#800000", // 23,  83 Maroon
    "#808000", // 29,  89 Olive
    "#008000", // 31,  97 Green
    "#800080", // 37, 101 Purple
    "#008080", // 41, 103 Teal
    "#000080", // 43, 107 Navy
];

/// The fill style is currently hard-coded, so the compiler helpfully points out
/// that only one of the variants is actually in use for any given build. An
/// alternative to suppressing the lint would be to allow changes at runtime.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum FillStyle {
    Color,
    Grayscale,
}

impl FillStyle {
    pub fn get(self, index: usize) -> String {
        match self {
            FillStyle::Color => COLORS[index % COLORS.len()].to_owned(),
            FillStyle::Grayscale => format!("#{i:02x}{i:02x}{i:02x}", i = 15 + index % 16 * 14),
        }
    }
}
