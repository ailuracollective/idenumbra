//! Country code constants (ISO 3166-1 alpha-3).

/// Country code for ARG:.
pub const ARG: &str = "ARG";
/// Country code for BOL:.
pub const BOL: &str = "BOL";
/// Country code for BRA:.
pub const BRA: &str = "BRA";
/// Country code for CHL:.
pub const CHL: &str = "CHL";
/// Country code for COL:.
pub const COL: &str = "COL";
/// Country code for CRI:.
pub const CRI: &str = "CRI";
/// Country code for CUB:.
pub const CUB: &str = "CUB";
/// Country code for DOM:.
pub const DOM: &str = "DOM";
/// Country code for ECU:.
pub const ECU: &str = "ECU";
/// Country code for GTM:.
pub const GTM: &str = "GTM";
/// Country code for HND:.
pub const HND: &str = "HND";
/// Country code for MEX:.
pub const MEX: &str = "MEX";
/// Country code for NIC:.
pub const NIC: &str = "NIC";
/// Country code for PAN:.
pub const PAN: &str = "PAN";
/// Country code for PER:.
pub const PER: &str = "PER";
/// Country code for PRY:.
pub const PRY: &str = "PRY";
/// Country code for SLV:.
pub const SLV: &str = "SLV";
/// Country code for URY:.
pub const URY: &str = "URY";
/// Country code for VEN:.
pub const VEN: &str = "VEN";

/// Returns canonical country code.
pub fn canonical(code: &str) -> Option<&'static str> {
    let code = code.trim();
    if code.eq_ignore_ascii_case("ARG") {
        Some(ARG)
    } else if code.eq_ignore_ascii_case("BOL") {
        Some(BOL)
    } else if code.eq_ignore_ascii_case("BRA") {
        Some(BRA)
    } else if code.eq_ignore_ascii_case("CHL") {
        Some(CHL)
    } else if code.eq_ignore_ascii_case("COL") {
        Some(COL)
    } else if code.eq_ignore_ascii_case("CRI") {
        Some(CRI)
    } else if code.eq_ignore_ascii_case("CUB") {
        Some(CUB)
    } else if code.eq_ignore_ascii_case("DOM") {
        Some(DOM)
    } else if code.eq_ignore_ascii_case("ECU") {
        Some(ECU)
    } else if code.eq_ignore_ascii_case("GTM") {
        Some(GTM)
    } else if code.eq_ignore_ascii_case("HND") {
        Some(HND)
    } else if code.eq_ignore_ascii_case("MEX") {
        Some(MEX)
    } else if code.eq_ignore_ascii_case("NIC") {
        Some(NIC)
    } else if code.eq_ignore_ascii_case("PAN") {
        Some(PAN)
    } else if code.eq_ignore_ascii_case("PER") {
        Some(PER)
    } else if code.eq_ignore_ascii_case("PRY") {
        Some(PRY)
    } else if code.eq_ignore_ascii_case("SLV") {
        Some(SLV)
    } else if code.eq_ignore_ascii_case("URY") {
        Some(URY)
    } else if code.eq_ignore_ascii_case("VEN") {
        Some(VEN)
    } else {
        None
    }
}
