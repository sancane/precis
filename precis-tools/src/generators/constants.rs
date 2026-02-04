//! Shared constants used across generators

// ASCII7 range boundaries
// 9.11. ASCII7 (K): cp is in {0021..007E}
pub(crate) const ASCII7_START: u32 = 0x0021; // ! (EXCLAMATION MARK)
pub(crate) const ASCII7_END: u32 = 0x007E; // ~ (TILDE)

// Canonical combining class for Virama characters
pub(crate) const CANONICAL_COMBINING_CLASS_VIRAMA: u8 = 9;
