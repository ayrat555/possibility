extern crate yaml_rust;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

const TAG_PROBABILITIES_PATH: &str = "./src/data/tags.yml";
const WORDS_PATH: &str = "./src/data/words.yml";
const ERROR_MESSAGE: &str = "yml file is invalid";

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Tag {
    CC,   // Conjunction, coordinating
    CD,   // Adjective, cardinal number
    DET,  // Determiner
    EX,   // Pronoun, existential there
    FW,   // Foreign words
    IN,   // Preposition / Conjunction
    JJ,   // Adjective
    JJR,  // Adjective, comparative
    JJS,  // Adjective, superlative
    LS,   // Symbol, list item
    MD,   // Verb, modal
    NN,   // Noun
    NNP,  // Noun, proper
    NNPS, // Noun, proper, plural
    NNS,  // Noun, plural
    PDT,  // Determiner, prequalifier
    POS,  // Possessive
    PRP,  // Determiner, possessive second
    PRPS, // Determiner, possessive
    RB,   // Adverb
    RBR,  // Adverb, comparative
    RBS,  // Adverb, superlative
    RP,   // Adverb, particle
    SYM,  // Symbol
    TO,   // Preposition
    UH,   // Interjection
    VB,   // Verb, infinitive
    VBD,  // Verb, past tense
    VBG,  // Verb, gerund
    VBN,  // Verb, past/passive participle
    VBP,  // Verb, base present form
    VBZ,  // Verb, present 3SG -s form
    WDT,  // Determiner, question
    WP,   // Pronoun, question
    WPS,  // Determiner, possessive & question
    WRB,  // Adverb, question
    PP,   // Punctuation, sentence ender
    PPC,  // Punctuation, comma
    PPD,  // Punctuation, dollar sign
    PPL,  // Punctuation, quotation mark left
    PPR,  // Punctuation, quotation mark right
    PPS,  // Punctuation, colon, semicolon, elipsis
    LRB,  // Punctuation, left bracket
    RRB,  // Punctuation, right bracket
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
    pub tag_probabilities: HashMap<Tag, HashMap<Tag, f32>>,
    pub words: HashMap<String, HashMap<Tag, i32>>,
}

struct Possibility {
    pub tag_data: TagData,
}

impl Possibility {
    pub fn new() -> Possibility {
        let tag_data = read_tag_data();

        Possibility { tag_data }
    }
}

fn read_tag_data() -> TagData {
    let tag_probabilities = read_tag_probabilities();
    let words = read_words();

    TagData {
        tag_probabilities,
        words,
    }
}

fn read_tag_probabilities() -> HashMap<Tag, HashMap<Tag, f32>> {
    let mut file = File::open(TAG_PROBABILITIES_PATH).unwrap();

    let mut string_probabilities = String::new();
    file.read_to_string(&mut string_probabilities).unwrap();

    let yml = YamlLoader::load_from_str(&string_probabilities).unwrap();

    let mut tag_probabilities_vec = yml
        .iter()
        .map(|yml_entry| match yml_entry {
            Yaml::Hash(tag_probabilities) => tag_probabilities
                .iter()
                .map(|(key, val)| {
                    let tag_key = match key {
                        Yaml::String(string_key) => Tag::from_str(string_key).unwrap(),
                        _ => panic!(ERROR_MESSAGE),
                    };

                    let values = match val {
                        Yaml::Hash(probabilities) => probabilities
                            .iter()
                            .map(|(key, val)| {
                                let tag_key = match key {
                                    Yaml::String(string_key) => Tag::from_str(string_key).unwrap(),
                                    _ => panic!(ERROR_MESSAGE),
                                };

                                let real_val = match val {
                                    Yaml::Real(string_val) => f32::from_str(string_val).unwrap(),
                                    _ => panic!(ERROR_MESSAGE),
                                };

                                (tag_key, real_val)
                            }).collect::<HashMap<Tag, f32>>(),
                        _ => panic!(ERROR_MESSAGE),
                    };

                    (tag_key, values)
                }).collect::<HashMap<Tag, HashMap<Tag, f32>>>(),

            _ => panic!(ERROR_MESSAGE),
        }).collect::<Vec<HashMap<Tag, HashMap<Tag, f32>>>>();

    tag_probabilities_vec.pop().unwrap()
}

fn read_words() -> HashMap<String, HashMap<Tag, i32>> {
    let mut file = File::open(WORDS_PATH).unwrap();

    let mut string_probabilities = String::new();
    file.read_to_string(&mut string_probabilities).unwrap();

    let yml = YamlLoader::load_from_str(&string_probabilities).unwrap();

    let mut words_vec = yml
        .iter()
        .map(|yml_entry| match yml_entry {
            Yaml::Hash(words) => words
                .iter()
                .map(|(key, val)| {
                    let word = match key {
                        Yaml::String(string_key) => string_key.clone(),
                        Yaml::Boolean(boolean_value) => if *boolean_value {
                            String::from("true")
                        } else {
                            String::from("false")
                        },
                        _ => panic!(ERROR_MESSAGE),
                    };

                    let values = match val {
                        Yaml::Hash(tags) => tags
                            .iter()
                            .map(|(key, val)| {
                                let tag_key = match key {
                                    Yaml::String(string_key) => Tag::from_str(string_key).unwrap(),
                                    _ => panic!(ERROR_MESSAGE),
                                };

                                let integer_val = match val {
                                    Yaml::Integer(i64_val) => *i64_val as i32,
                                    _ => panic!(ERROR_MESSAGE),
                                };

                                (tag_key, integer_val)
                            }).collect::<HashMap<Tag, i32>>(),
                        _ => panic!(ERROR_MESSAGE),
                    };

                    (word, values)
                }).collect::<HashMap<String, HashMap<Tag, i32>>>(),

            _ => panic!(ERROR_MESSAGE),
        }).collect::<Vec<HashMap<String, HashMap<Tag, i32>>>>();

    words_vec.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::Possibility;
    use super::Tag::*;

    #[test]
    fn initializes_pos_tagger_with_data_from_yml_file() {
        let possibility = Possibility::new();

        assert_eq!(possibility.tag_data.tag_probabilities.len(), 44);

        assert_eq!(
            possibility
                .tag_data
                .tag_probabilities
                .get(&CC)
                .unwrap()
                .len(),
            40
        );

        assert_eq!(
            possibility
                .tag_data
                .tag_probabilities
                .get(&CD)
                .unwrap()
                .get(&DET)
                .unwrap()
                .clone(),
            0.0292094 as f32
        );
    }
}
