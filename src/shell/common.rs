use crate::config::Config;

pub trait Shell {
    fn name(&self) -> &'static str;
    fn try_configure(&self, config: &Config) -> Result<(), String>;

    fn configure(&self, config: &Config) {
        println!("Setting up {}", self.name());
        match self.try_configure(config) {
            Ok(()) => println!("Successfully set up {}", self.name()),
            Err(msg) => println!("Erring setting up {}: {}", self.name(), msg),
        };
    }
}

pub fn replace_file_content(
    existing_content: Vec<String>,
    new_content: &Vec<String>,
) -> Vec<String> {
    assert!(new_content.len() > 0);
    let fr = existing_content
        .iter()
        .position(|x| x.trim() == new_content[0].trim());
    if let Some(fr) = fr {
        let last_line = new_content
            .last()
            .expect("already asserted new_content.len() > 0")
            .trim();
        let to = existing_content
            .iter()
            .skip(fr)
            .position(|x| x.trim() == last_line);
        if let Some(to) = to {
            let to = to + fr;
            let mut updated_content = existing_content;
            let suffix = updated_content[to + 1..].to_owned();
            updated_content.truncate(fr);
            updated_content.extend(new_content.clone());
            updated_content.extend_from_slice(&suffix);
            return updated_content;
        }
    }
    let mut existing_content = existing_content;
    existing_content.extend_from_slice(new_content);
    existing_content
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_replace_file_content() {
        let existing_content: Vec<String> = vec!["0", "1", "2", "X", "A", "B", "C", "D", "Y"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let new_content: Vec<String> = vec!["X", "a", "b", "c", "Y"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let expected: Vec<String> = vec!["0", "1", "2", "X", "a", "b", "c", "Y"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let updated_content = replace_file_content(existing_content, &new_content);
        assert_eq!(expected, updated_content);

        let existing_content: Vec<String> = vec!["X", "1", "2", "3", "Y"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let new_content = existing_content.clone();
        let expected = existing_content.clone();
        let updated_content = replace_file_content(existing_content, &new_content);
        assert_eq!(expected, updated_content);
    }
}
