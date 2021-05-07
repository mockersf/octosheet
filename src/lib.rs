use serde::Deserialize;
use serde_xml_rs::Deserializer;

#[derive(Deserialize, Debug)]
struct Sheet {
    identification: Identification,
    #[serde(rename = "credit", default)]
    credits: Vec<Credit>,
    #[serde(rename = "part-list")]
    part_list: PartList,
    #[serde(rename = "part")]
    parts: Vec<Part>,
}

#[derive(Deserialize, Debug)]
struct Identification {
    creator: Creator,
}

#[derive(Deserialize, Debug)]
struct Credit {
    page: u8,
    #[serde(rename = "credit-words")]
    words: Vec<CreditWords>,
}

#[derive(Deserialize, Debug)]
struct CreditWords {
    #[serde(rename = "$value")]
    words: String,
}

#[derive(Deserialize, Debug)]
struct Creator {
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
struct Pitch {
    step: String,
    alter: Option<i8>,
    octave: u8,
}

#[derive(Deserialize, Debug)]
struct Beam {
    number: u8,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Deserialize, Debug)]
struct PartList {
    #[serde(rename = "score-part")]
    part_list: Vec<ScorePart>,
}

#[derive(Deserialize, Debug)]
struct ScorePart {
    id: String,
    #[serde(rename = "part-name")]
    name: String,
    #[serde(rename = "part-abbreviation")]
    abbreviation: String,
    #[serde(rename = "score-instrument")]
    instrument: Instrument,
}

#[derive(Deserialize, Debug)]
struct Instrument {
    id: String,
    #[serde(rename = "instrument-name")]
    name: String,
}

#[derive(Deserialize, Debug)]
struct Part {
    id: String,
    #[serde(rename = "measure")]
    measures: Vec<Measure>,
}

#[derive(Deserialize, Debug)]
struct Measure {
    number: u8,
    #[serde(rename = "$value")]
    notes: Vec<Note>,
}

#[derive(Deserialize, Debug)]
struct Key {
    fifths: i8,
}

#[derive(Deserialize, Debug)]
struct Time {
    beats: u8,
    #[serde(rename = "beat-type")]
    beat_type: u8,
}

#[derive(Deserialize, Debug)]
struct Clef {
    number: u8,
    sign: String,
    line: String,
}

pub fn parse_file(path: &str) -> () {
    // let mut de = Deserializer::new_from_reader(std::fs::File::open(path).unwrap())
    //     .non_contiguous_seq_elements(true);
    // let actual = dbg!(Measure::deserialize(&mut de)).unwrap();
    dbg!(serde_xml_rs::from_reader::<_, Sheet>(
        std::fs::File::open(path).unwrap()
    ));
}
