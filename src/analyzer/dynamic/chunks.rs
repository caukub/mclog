use crate::analyzer::DynamicAnalyzerDetails;
use regex::Regex;
use rhai::{Dynamic, ImmutableString};
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Clone)]
pub struct Chunks {
    dad: DynamicAnalyzerDetails,
}

impl Chunks {
    pub fn new(dad: DynamicAnalyzerDetails) -> Self {
        Self { dad }
    }

    pub fn has_line(self, to_find: ImmutableString) -> bool {
        self.dad.chunks.iter().any(|line| line.contains(&*to_find))
    }

    pub fn has_line2(self, to_find: ImmutableString, identifier: String) -> Dynamic {
        // TODO (binding, escape)
        let regex = REPLACE_NUMS_REGEX.replace_all(to_find.as_str(), "(.*)");
        let regex = Regex::new(&regex).unwrap();

        let mut capturess: Captures = Captures {
            captures: HashMap::new(),
            identifier,
        };

        let mut capturesss: Vec<Captures> = Vec::new();

        let mut capture_index = 0;

        for line in self.dad.chunks {
            if regex.is_match(&line) {
                let captures = regex.captures(&line).unwrap();

                for idx in 0..captures.len() {
                    if idx == 0 {
                        continue;
                    }

                    let capture = captures.get(idx).unwrap();
                    let capture = capture.as_str().to_string();

                    capturess.captures.insert(capture_index, capture);

                    capture_index += 1;

                    if (capture_index + 1) as usize == captures.len() {
                        capturesss.push(capturess.clone());
                        capture_index = 0;
                    }
                }
            }
        }

        if capturess.captures.is_empty() {
            Dynamic::UNIT
        } else {
            Dynamic::from(capturesss)
        }
    }

    pub fn has_line_permissive(self, to_find: ImmutableString) -> bool {
        self.dad
            .chunks
            .iter()
            .any(|line| line.to_lowercase().contains(&to_find.to_lowercase()))
    }
}

static REPLACE_NUMS_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{\d}").unwrap_or_else(|err| {
        panic!("Failed to create 'REPLACE_NUMS_REGEX: {}'", err);
    })
});

#[derive(Clone, Debug)]
pub struct Captures {
    pub identifier: String,
    pub captures: HashMap<i32, String>,
}
