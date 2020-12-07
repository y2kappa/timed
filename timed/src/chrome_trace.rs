use crate::hop::Hop;

pub fn from(hops: &[Hop]) -> String {

    let result = hops
        .iter()
        .map(|hop| format!("\t{}", hop.to_chrome_trace()))
        .collect::<Vec<String>>()
        .join(",\n");

    format!("[\n{}\n]", result)
}