use anyhow::{Context, Result};
use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

/// The Corruptor takes a word, and generates a new word
/// by applying Grimm's Law-style patterns. You can drive it one
/// step at a time, or you can drive it for a number of iterations.
/// Think of an iteration being a move like 'what would this name
/// be like in the next country over?'
struct Corruptor {
    patterns: Vec<(String, String)>,
    cursor: usize,
}

impl Corruptor {
    /// construct a new Corruptor with the given patterns
    fn new(patterns: Vec<(String, String)>) -> Self {
        let cursor = patterns.len();
        Corruptor { patterns, cursor }
    }

    /// corrupt the given name once, and return the new name
    fn corrupt_once(&mut self, name: &str) -> String {
        let mut name = name.to_string();

        // we want to cycle through the available patterns -- so if we have 10 patterns,
        // we would start by applying pattern 0, then 1, then 2, until one of them changes
        // the name. E.g., if pattern 0 is (d => t) and we apply it to 'david', we get 'tavit'.
        // But if we change 'susan' get set 'susan' back, and we should try another one. We
        // advance until either we complete a loop with no change (the failure state, so to
        // speak) or we find a new variant of the name.
        //
        // We've now left the corruptor with a new cursor position, so if you try again with the
        // same name it tries different patterns.
        let starting_cursor = self.cursor;

        // advance the cursor through the patterns, looping if necessary
        let mut cursor = (starting_cursor + 1) % self.patterns.len();

        // keep going until we have tried applying all the patterns;
        while cursor != starting_cursor {
            // grab a pattern like ("d", "t")
            let (pattern, replacement) = &self.patterns[cursor];

            // replace it - eg "david".replace("d", "t") => "tavit"
            let new_name = name.replace(pattern, replacement);
            if new_name != name {
                // if the name changed, we're done
                self.cursor = cursor;
                return Self::relax(new_name);
            }

            // if not, keep going with the next pattern
            cursor = (cursor + 1) % self.patterns.len();
        }

        name
    }

    /// apply several iterations of corrupt_once
    fn corrupt(&mut self, name: &str, iterations: usize) -> String {
        let mut remaining_iterations = iterations;
        let mut name = name.to_string();
        while remaining_iterations > 0 {
            name = self.corrupt_once(&name);
            remaining_iterations -= 1;
        }
        name
    }

    // sometimes the rules end up with silliness, like "alffffffffonzo". The relax method
    // is supposed to 'chill out' the name a little bit to make it more real
    fn relax(name: impl Into<String>) -> String {
        /// the only pattern we recognize is three of the same letter in a row, like "aaa"
        let mut name = name.into();
        for c in 'a'..'z' {
            let search = format!("{}{}{}", c, c, c);
            let replacement = format!("{}{}", c, c);
            name = name.replace(&search, &replacement);
        }
        name
    }
}

/// Parse a complex pattern string into pairs -- example mighe be
/// "bh => b => p => f; dh => d => t => th"
/// this contains two sequences (note the semicolon) and the sequences
/// contain pairs lie bh->b, b->p, etc.
fn parse_patterns(input: &str) -> Result<Vec<(String, String)>> {
    let mut patterns = Vec::new();

    // split on semicolon, then split on "=>" and trim whitespace
    for sequence in input.split(";").map(|l| l.trim()) {
        let mut parts: VecDeque<&str> = sequence.split("=>").map(|l| l.trim()).collect();
        if parts.len() == 0 {
            continue;
        }
        // a zip algorithm which pairs up adjascent elements
        let mut prev_part = parts
            .pop_front()
            .context("should have at least one entry")?;
        while parts.len() > 0 {
            let next_part = parts.pop_front().context("I just checked that!")?;
            patterns.push((prev_part.to_string(), next_part.to_string()));
            prev_part = next_part;
        }
    }
    Ok(patterns)
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::*;
    use std::path::PathBuf;

    fn name_list() -> Vec<String> {
        let content = include_str!("./name-list.txt");
        content.lines().map(|l| l.to_string()).collect()
    }

    #[test]
    fn parse_pattern() -> Result<()> {
        // Grimm's Law
        let input = r#"
            bh => b => p => f; 
            dh => d => t => th; 
            gh => g => k => x; 
            gwh => gw => kw => xw"#;
        let expected: Vec<(String, String)> = vec![
            ("bh", "b"),
            ("b", "p"),
            ("p", "f"),
            ("dh", "d"),
            ("d", "t"),
            ("t", "th"),
            ("gh", "g"),
            ("g", "k"),
            ("k", "x"),
            ("gwh", "gw"),
            ("gw", "kw"),
            ("kw", "xw"),
        ]
        .into_iter()
        .map(|(a, b)| (a.into(), b.into()))
        .collect();
        let actual = parse_patterns(input)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn corrupt_agatha() -> Result<()> {
        let patterns = parse_patterns("th => ff")?;
        let mut corruptor = Corruptor::new(patterns);
        let corrupted_name = corruptor.corrupt("agatha", 1);
        assert_eq!(corrupted_name, "agaffa");
        Ok(())
    }

    #[test]
    fn corrupt_david() -> Result<()> {
        let patterns = parse_patterns("d => t")?;
        let mut corruptor = Corruptor::new(patterns);
        let corrupted_name = corruptor.corrupt_once("david");
        assert_eq!(corrupted_name, "tavit");
        Ok(())
    }

    #[test]
    fn corrupt_several() -> Result<()> {
        let patterns = parse_patterns(
            r#"
            pir => per;
            ie => iey; 
            sa => za => tsa => tzah; 
            th => dd => t;
            gnu => gnae;
            cel => ciel => sel => tzel;
            lot => lod;
            ric => rick => rik => rijk;
            ph => ff => f => v => vh; 
            na => ne;
            er => aer;
            dwa => dva => tva => cha; 
            ao => ai => aiwa => awa => a; 
            d => t; 
            tta => tva; 
            lle => lla => llya;
            in => en => un => um => ium;
            i => ih => y;
            por => pro;
            b => p => f;
            co => ko => kho;
            an => in => ain;
            zu => tzu; 
            ace => ache => eiche;
            tt => t;
            ys => iz => it => itz => its => itsa => itsah; 
            ia => aya; 
            ena => ina => iyna; 
            era => ira => idra; 
            ick => ich => ech => eckh
            "#,
        )?;
        let mut corruptor = Corruptor::new(patterns);
        let mut remaining = 0;
        const N: usize = 4;
        let mut readme_content = "# Name Corruptor\n\n".to_string();

        for name in name_list() {
            let name = name.to_lowercase();
            let mut generated = vec![name.clone()];
            let mut message = format!("    {:12}", name);
            for i in 0..N {
                let next = &generated[generated.len() - 1];
                let corrupted = corruptor.corrupt_once(next);
                generated.push(corrupted.clone());
                message.push_str(&format!(" => {:12}", corrupted));
            }
            let final_name = &generated[generated.len() - 1];
            readme_content.push_str(&format!("{}\n", message));
            let message = if final_name == &name {
                remaining += 1;
                message.red()
            } else {
                message.green()
            };
            eprintln!("{}", message);
        }
        assert_eq!(remaining, 0, "didn't managed to corrupt all names");

        readme_content.push_str("```\n");
        let readme_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("README.md");
        let old_readme = std::fs::read_to_string(&readme_path)?;
        if old_readme != readme_content {
            std::fs::write(readme_path, readme_content)?;
        }
        Ok(())
    }
}
