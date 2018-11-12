extern crate yaml_rust;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

const TAG_PROBABILITIES_PATH: &str = "./src/data/tags.yml";

#[derive(Hash, PartialEq, Eq, Debug)]
enum Tag {
    CC,
    CD,
    DET,
    EX,
    FW,
    IN,
    JJ,
    JJR,
    JJS,
    LS,
    MD,
    NN,
    NNP,
    NNPS,
    NNS,
    PDT,
    POS,
    PRP,
    PRPS,
    RB,
    RBR,
    RBS,
    RP,
    SYM,
    TO,
    UH,
    VB,
    VBD,
    VBG,
    VBN,
    VBP,
    VBZ,
    WDT,
    WP,
    WPS,
    WRB,
    PP,
    PPC,
    PPD,
    PPL,
    PPR,
    PPS,
    LRB,
    RRB,
}

impl FromStr for Tag {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, String> {
        match string {
            "cc" => Ok(Tag::CC),
            "cd" => Ok(Tag::CD),
            "det" => Ok(Tag::DET),
            "ex" => Ok(Tag::EX),
            "fw" => Ok(Tag::FW),
            "in" => Ok(Tag::IN),
            "jj" => Ok(Tag::JJ),
            "jjr" => Ok(Tag::JJR),
            "jjs" => Ok(Tag::JJS),
            "ls" => Ok(Tag::LS),
            "md" => Ok(Tag::MD),
            "nn" => Ok(Tag::NN),
            "nnp" => Ok(Tag::NNP),
            "nnps" => Ok(Tag::NNPS),
            "nns" => Ok(Tag::NNS),
            "pdt" => Ok(Tag::PDT),
            "pos" => Ok(Tag::POS),
            "prp" => Ok(Tag::PRP),
            "prps" => Ok(Tag::PRPS),
            "rb" => Ok(Tag::RB),
            "rbr" => Ok(Tag::RBR),
            "rbs" => Ok(Tag::RBS),
            "rp" => Ok(Tag::RP),
            "sym" => Ok(Tag::SYM),
            "to" => Ok(Tag::TO),
            "uh" => Ok(Tag::UH),
            "vb" => Ok(Tag::VB),
            "vbd" => Ok(Tag::VBD),
            "vbg" => Ok(Tag::VBG),
            "vbn" => Ok(Tag::VBN),
            "vbp" => Ok(Tag::VBP),
            "vbz" => Ok(Tag::VBZ),
            "wdt" => Ok(Tag::WDT),
            "wp" => Ok(Tag::WP),
            "wps" => Ok(Tag::WPS),
            "wrb" => Ok(Tag::WRB),
            "pp" => Ok(Tag::PP),
            "ppc" => Ok(Tag::PPC),
            "ppd" => Ok(Tag::PPD),
            "ppl" => Ok(Tag::PPL),
            "ppr" => Ok(Tag::PPR),
            "pps" => Ok(Tag::PPS),
            "lrb" => Ok(Tag::LRB),
            "rrb" => Ok(Tag::RRB),
            wrong_tag => Err(format!("Invalid tag: {}", wrong_tag)),
        }
    }
}

struct TagData {
    tag_probabilities: HashMap<Tag, HashMap<Tag, f32>>,
}

struct Possibility {
    tag_data: TagData,
}

impl Possibility {
    pub fn new() -> Possibility {
        let tag_data = read_tag_data();

        Possibility { tag_data }
    }
}

fn read_tag_data() -> TagData {
    let mut file = File::open(TAG_PROBABILITIES_PATH).unwrap();

    let mut string_probabilities = String::new();
    file.read_to_string(&mut string_probabilities).unwrap();

    let hash_tag_probabilities = YamlLoader::load_from_str(&string_probabilities).unwrap();

    parse_yaml(hash_tag_probabilities)
}

fn parse_yaml(yml: Vec<Yaml>) -> TagData {
    let tag_probabilities_vec = yml
        .clone()
        .iter()
        .map(|yml_entry| match yml_entry {
            Yaml::Hash(tag_probabilities) => tag_probabilities
                .iter()
                .map(|(key, val)| {
                    let string_key = match key {
                        Yaml::String(string_key) => Tag::from_str(string_key).unwrap(),
                        _ => panic!("tags.yml file is invalid"),
                    };

                    let values = match val {
                        Yaml::Hash(probabilities) => probabilities
                            .iter()
                            .map(|(key, val)| {
                                let string_key = match key {
                                    Yaml::String(string_key) => Tag::from_str(string_key).unwrap(),
                                    _ => panic!("tags.yml file is invalid"),
                                };

                                let string_val = match val {
                                    Yaml::Real(string_val) => f32::from_str(string_val).unwrap(),
                                    _ => panic!("tags.yml file is invalid"),
                                };

                                (string_key, string_val)
                            }).collect::<HashMap<Tag, f32>>(),
                        _ => panic!("tags.yml file is invalid"),
                    };

                    (string_key, values)
                }).collect::<HashMap<Tag, HashMap<Tag, f32>>>(),

            _ => panic!("tags.yml file is invalid"),
        }).collect::<Vec<HashMap<Tag, HashMap<Tag, f32>>>>();

    TagData {
        tag_probabilities: tag_probabilities_vec.first().unwrap().clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::Possibility;

    #[test]
    fn initializes_pos_tagger() {
        let possibility = Possibility::new();
    }

}
