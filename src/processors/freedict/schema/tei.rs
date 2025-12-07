use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TEI {
    pub text: Text,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub body: Body,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    #[serde(rename = "entry", default)]
    pub entries: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    #[serde(rename = "@xml:id", default)]
    pub id: Option<String>,

    #[serde(rename = "@type", default)]
    pub entry_type: Option<String>,

    #[serde(rename = "@sortKey", default)]
    pub sort_key: Option<String>,

    #[serde(default)]
    pub form: Vec<Form>,

    #[serde(rename = "gramGrp", default)]
    pub gram_grp: Vec<GramGrp>,

    #[serde(default)]
    pub sense: Vec<Sense>,

    #[serde(default)]
    pub hom: Vec<Hom>,

    #[serde(default)]
    pub def: Vec<Definition>,

    #[serde(default)]
    pub etym: Vec<Etym>,

    #[serde(default)]
    pub usg: Vec<Usg>,

    #[serde(default)]
    pub xr: Vec<Xr>,

    #[serde(default)]
    pub cit: Vec<Cit>,

    #[serde(default)]
    pub re: Vec<Re>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[serde(rename = "@type", default)]
    pub form_type: Option<String>,

    #[serde(default)]
    pub orth: Vec<Orth>,

    #[serde(default)]
    pub pron: Vec<Pron>,

    #[serde(default)]
    pub hyph: Vec<Hyph>,

    #[serde(default)]
    pub syll: Vec<Syll>,

    #[serde(default)]
    pub stress: Vec<Stress>,

    #[serde(rename = "gramGrp", default)]
    pub gram_grp: Vec<GramGrp>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Orth {
    #[serde(rename = "@type", default)]
    pub orth_type: Option<String>,

    #[serde(rename = "@extent", default)]
    pub extent: Option<String>,

    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pron {
    #[serde(rename = "@extent", default)]
    pub extent: Option<String>,

    #[serde(rename = "@notation", default)]
    pub notation: Option<String>,

    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hyph {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Syll {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stress {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GramGrp {
    #[serde(rename = "@type", default)]
    pub gram_type: Option<String>,

    #[serde(default)]
    pub pos: Vec<Pos>,

    #[serde(default, rename = "gen")]
    pub gender: Vec<Gen>,

    #[serde(default)]
    pub number: Vec<Number>,

    #[serde(default)]
    pub case: Vec<Case>,

    #[serde(default)]
    pub per: Vec<Per>,

    #[serde(default)]
    pub tns: Vec<Tns>,

    #[serde(default)]
    pub mood: Vec<Mood>,

    #[serde(default)]
    #[serde(rename = "iType")]
    pub i_type: Vec<IType>,

    #[serde(default)]
    pub gram: Vec<Gram>,

    #[serde(default)]
    pub subc: Vec<Subc>,

    #[serde(default)]
    pub colloc: Vec<Colloc>,

    #[serde(default)]
    pub usg: Vec<Usg>,

    #[serde(default)]
    pub lbl: Vec<Lbl>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pos {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gen {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Number {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Case {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Per {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tns {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mood {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IType {
    #[serde(rename = "@type", default)]
    pub itype_type: Option<String>,

    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gram {
    #[serde(rename = "@type", default)]
    pub gram_type: Option<String>,

    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subc {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Colloc {
    #[serde(rename = "@type", default)]
    pub colloc_type: Option<String>,

    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sense {
    #[serde(rename = "@xml:id", default)]
    pub id: Option<String>,

    #[serde(rename = "@level", default)]
    pub level: Option<String>,

    #[serde(rename = "@n", default)]
    pub n: Option<String>,

    #[serde(default)]
    pub def: Vec<Definition>,

    #[serde(default)]
    pub usg: Vec<Usg>,

    #[serde(default)]
    pub cit: Vec<Cit>,

    #[serde(default)]
    pub xr: Vec<Xr>,

    #[serde(default)]
    pub sense: Vec<Sense>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usg {
    #[serde(rename = "@type", default)]
    pub usg_type: Option<String>,

    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cit {
    #[serde(rename = "@type", default)]
    pub cit_type: Option<String>,

    #[serde(rename = "@xml:lang", default)]
    pub lang: Option<String>,

    #[serde(default)]
    pub quote: Vec<Quote>,

    #[serde(default)]
    pub q: Vec<Q>,

    #[serde(default)]
    pub cit: Vec<Cit>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Q {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Xr {
    #[serde(rename = "@type", default)]
    pub xr_type: Option<String>,

    #[serde(default)]
    pub lbl: Vec<Lbl>,

    #[serde(default)]
    pub ref_: Vec<Ref>,

    #[serde(default)]
    pub ptr: Vec<Ptr>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lbl {
    #[serde(rename = "@type", default)]
    pub lbl_type: Option<String>,

    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ref {
    #[serde(rename = "@target", default)]
    pub target: Option<String>,

    #[serde(rename = "@type", default)]
    pub ref_type: Option<String>,

    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ptr {
    #[serde(rename = "@target")]
    pub target: String,

    #[serde(rename = "@type", default)]
    pub ptr_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Etym {
    #[serde(rename = "$value")]
    pub content: String,

    #[serde(default)]
    pub lang: Vec<Lang>,

    #[serde(default)]
    pub mentioned: Vec<Mentioned>,

    #[serde(default)]
    pub lbl: Vec<Lbl>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lang {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mentioned {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hom {
    #[serde(default)]
    pub form: Vec<Form>,

    #[serde(rename = "gramGrp", default)]
    pub gram_grp: Vec<GramGrp>,

    #[serde(default)]
    pub sense: Vec<Sense>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Re {
    #[serde(rename = "@type", default)]
    pub re_type: Option<String>,

    #[serde(default)]
    pub form: Vec<Form>,

    #[serde(default)]
    pub sense: Vec<Sense>,
}
