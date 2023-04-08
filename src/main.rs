use anyhow::{Context, Result};
use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

struct Corruptor {
    patterns: Vec<(String, String)>,
    cursor: usize,
}

impl Corruptor {
    fn new(patterns: Vec<(String, String)>) -> Self {
        Corruptor {
            patterns,
            cursor: 0,
        }
    }

    fn corrupt_once(&mut self, name: &str) -> String {
        let mut name = name.to_string();

        let starting_cursor = self.cursor;
        let mut cursor = (starting_cursor + 1) % self.patterns.len();
        while cursor != starting_cursor {
            let (pattern, replacement) = &self.patterns[cursor];
            let new_name = name.replace(pattern, replacement);
            if new_name != name {
                self.cursor = cursor;
                return Self::relax(new_name);
            }
            cursor = (cursor + 1) % self.patterns.len();
        }

        name
    }

    fn corrupt(&mut self, name: &str, iterations: usize) -> String {
        let mut remaining_iterations = iterations;
        let mut name = name.to_string();
        while remaining_iterations > 0 {
            name = self.corrupt_once(&name);
            remaining_iterations -= 1;
        }
        name
    }

    fn relax(name: impl Into<String>) -> String {
        let mut name = name.into();
        for c in 'a'..'z' {
            let search = format!("{}{}{}", c, c, c);
            let replacement = format!("{}{}", c, c);
            name = name.replace(&search, &replacement);
        }
        name
    }
}

fn parse_patterns(input: &str) -> Result<Vec<(String, String)>> {
    let mut patterns = Vec::new();

    for sequence in input.split(";").map(|l| l.trim()) {
        let mut parts: VecDeque<&str> = sequence.split("=>").map(|l| l.trim()).collect();
        if parts.len() == 0 {
            continue;
        }
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
