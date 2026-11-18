fn char_val(b: u8) -> u32 {
    // Each character is worth its ASCII code minus 48: digits keep 0-9 and
    // A-Z become 17-42 (IN RFB 2.229/2024, "Manual de Cálculo do DV").
    (b as u32) - 48
}
