use lrwn_filters::EuiPrefix;
use std::str::FromStr;
fn main() {
    let prefix = EuiPrefix::from_str("0100000000000000/8").unwrap();
    let _ = prefix.is_match([1, 0, 0, 0, 0, 0, 0, 0]);
}
