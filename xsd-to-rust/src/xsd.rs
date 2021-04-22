use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Any {
    pub namespace: String,
    pub process_contents: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Attribute {
    pub name: String,
    pub r#type: String,
    pub r#use: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Choice {
    pub element: Vec<Element>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ComplexType {
    pub name: String,
    #[serde(rename = "$value")]
    pub kind: ComplexTypeKind,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum ComplexTypeKind {
    Choice(Choice),
    SimpleContent(SimpleContent),
    Sequence(Sequence),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Element {
    pub name: String,
    pub r#type: String,
    #[serde(default)]
    pub max_occurs: Option<String>,
    #[serde(default)]
    pub min_occurs: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Extension {
    pub base: String,
    pub attribute: Attribute,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Enumeration {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Pattern {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Restriction {
    pub base: String,
    #[serde(default)]
    pub fraction_digits: Option<String>,
    #[serde(default)]
    pub total_digits: Option<String>,
    #[serde(default)]
    pub min_inclusive: Option<String>,
    #[serde(default)]
    pub pattern: Option<Pattern>,
    #[serde(default)]
    pub enumeration: Option<Vec<Enumeration>>,
    #[serde(default)]
    pub min_length: Option<String>,
    #[serde(default)]
    pub max_length: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Schema {
    pub element_form_default: String,
    pub target_namespace: String,
    #[serde(rename = "element")]
    pub elements: Vec<Element>,
    #[serde(rename = "simpleType")]
    pub simple_types: Vec<SimpleType>,
    #[serde(rename = "complexType")]
    pub complex_types: Vec<ComplexType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Sequence {
    #[serde(default)]
    pub element: Vec<Element>,
    #[serde(default)]
    pub any: Option<Any>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SimpleContent {
    pub extension: Extension,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SimpleType {
    pub name: String,
    #[serde(rename = "$value")]
    pub kind: SimpleTypeKind,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum SimpleTypeKind {
    Restriction(Restriction),
}
