use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const TAG_PROBABILITIES_PATH: &str = "./src/data/tags.yml";

#[derive(Hash, PartialEq, Eq)]
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
        RRB
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
                        wrong_tag => Err(format!("Invalid tag: {}", wrong_tag))
                }
        }
}

struct TagProbabilty {
        tag: Tag,
        probabiity: f32
}

struct TagData {
        tag_probabilities: HashMap<Tag, Vec<TagProbabilty>>
}

struct Possibility {
        tag_data: TagData
}

impl Possibility {
        pub fn new()  -> Possibility {
                let tag_data = read_tag_data();

                Possibility{ tag_data }
        }
}

fn read_tag_data() -> TagData {
        let file = File::open(TAG_PROBABILITIES_PATH).unwrap();


        TagData{tag_probabilities: HashMap::new()}
}

#[cfg(test)]
mod tests {
        use super::Possibility;

}
