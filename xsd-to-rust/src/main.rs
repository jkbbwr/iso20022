mod xsd;

use std::fs::read_to_string;
use serde_xml_rs::Deserializer;
use crate::xsd::Schema;
use serde::Deserialize;


fn main() -> anyhow::Result<()> {
    let data = read_to_string("./xsd/pacs.008.001.09.xsd")?;
    let mut de = Deserializer::new_from_reader(data.as_bytes()).non_contiguous_seq_elements(true);
    let schema = Schema::deserialize(&mut de).unwrap();
    println!("{:?}", schema);
    Ok(())
}
