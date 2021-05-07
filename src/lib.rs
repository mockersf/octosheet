use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Sheet {
    identification: Identification,
    #[serde(rename = "credit", default)]
    credits: Vec<Credit>,
    #[serde(rename = "part-list")]
    part_list: PartList,
    #[serde(rename = "part")]
    parts: Vec<Part>,
}

#[derive(Deserialize, Debug)]
    creator: Creator,
pub struct Identification {
}

#[derive(Deserialize, Debug)]
pub struct Credit {
    page: u8,
    #[serde(rename = "credit-words")]
    words: Vec<CreditWords>,
}

#[derive(Deserialize, Debug)]
pub struct CreditWords {
    #[serde(rename = "$value")]
    words: String,
}

#[derive(Deserialize, Debug)]
pub struct Creator {
    #[serde(rename = "type")]
    ty: String,
    #[serde(rename = "$value")]
    name: String,
}

#[derive(Deserialize, Debug)]
enum Note {
    #[serde(rename = "note")]
    Note {
        pitch: Option<Pitch>,
        duration: u8,
        voice: u8,
        #[serde(rename = "type")]
        ty: Option<String>,
        stem: Option<String>,
        staff: u8,
        #[serde(rename = "beam")]
        beams: Option<Vec<Beam>>,
        dot: Option<Vec<()>>,
        accidental: Option<String>,
        rest: Option<bool>,
    },
    #[serde(rename = "backup")]
    Backup { duration: u8 },
    #[serde(rename = "forward")]
    Forward { duration: u8 },
    #[serde(rename = "direction")]
    Direction { staff: u8 },
    #[serde(rename = "print")]
    Print {},
    #[serde(rename = "barline")]
    Barline { location: String },
    #[serde(rename = "attributes")]
    Attributes {
        divisions: Option<u8>,
        key: Option<Key>,
        time: Option<Time>,
        staves: Option<u8>,
        #[serde(rename = "clef")]
        clefs: Option<Vec<Clef>>,
    },
}

#[derive(Deserialize, Debug)]
pub struct Pitch {
    step: String,
    alter: Option<i8>,
    octave: u8,
}

#[derive(Deserialize, Debug)]
pub struct Beam {
    number: u8,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Deserialize, Debug)]
pub struct PartList {
    #[serde(rename = "score-part")]
    part_list: Vec<ScorePart>,
}

#[derive(Deserialize, Debug)]
pub struct ScorePart {
    id: String,
    #[serde(rename = "part-name")]
    name: String,
    #[serde(rename = "part-abbreviation")]
    abbreviation: String,
    #[serde(rename = "score-instrument")]
    instrument: Instrument,
}

#[derive(Deserialize, Debug)]
pub struct Instrument {
    id: String,
    #[serde(rename = "instrument-name")]
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Part {
    id: String,
    #[serde(rename = "measure")]
    measures: Vec<Measure>,
}

#[derive(Deserialize, Debug)]
pub struct Measure {
    number: u8,
    #[serde(rename = "$value")]
    notes: Vec<Note>,
}

#[derive(Deserialize, Debug)]
pub struct Key {
    fifths: i8,
}

#[derive(Deserialize, Debug)]
pub struct Time {
    beats: u8,
    #[serde(rename = "beat-type")]
    beat_type: u8,
}

#[derive(Deserialize, Debug)]
pub struct Clef {
    number: u8,
    sign: String,
    line: String,
}

pub fn parse_file(path: &str) -> Result<Sheet, Box<dyn std::error::Error>> {
    Ok(serde_xml_rs::from_reader::<_, Sheet>(
        std::fs::File::open(path).unwrap(),
    )?)
}
