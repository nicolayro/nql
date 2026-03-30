#[derive(Debug, PartialEq)]
enum Command {
    Get(String),
    Set(String, String),
}

fn parse(input: &str) -> Option<Command> {
    let (cmd, rest) = input.split_once(" ")?;
    match cmd {
        "GET" => Some(Command::Get(rest.into())),
        "SET" => {
            let (key, value) = rest.split_once(" ")?;
            Some(Command::Set(key.into(), value.into()))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_none_when_input_is_empty() {
        let command = parse("");
        assert_eq!(command, None)
    }

    #[test]
    fn should_parse_get() {
        let key = String::from("name");
        let input = format!("GET {key}");

        let command = parse(&input);
        assert_eq!(command, Some(Command::Get(key)));
    }

    #[test]
    fn should_parse_set() {
        let key = String::from("name");
        let value = String::from("nql");
        let input = format!("SET {key} {value}");

        let command = parse(&input);
        assert_eq!(command, Some(Command::Set(key, value)));
    }
}
