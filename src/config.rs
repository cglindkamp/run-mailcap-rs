#[derive(Debug)]
#[derive(PartialEq)]
pub enum Action {
    View,
    Cat,
    Edit,
    Compose,
    Print,
}

impl Action {
    fn from(actionstr: &str) -> Action {
        match actionstr {
            "view" => Action::View,
            "cat" => Action::Cat,
            "edit" => Action::Edit,
            "compose" => Action::Compose,
            "print" => Action::Print,
            _ => Action::View,
        }
    }

    fn from_program_name(programname: &str) -> Action {
        let programname = programname.trim_end_matches("-rs");
        match programname {
            "see" => Action::View,
            "change" => Action::Edit,
            "create" => Action::Compose,
            _ => Action::from(programname),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub action: Action,
}

impl Config {
    pub fn parse<T>(args: T) -> Result<Config, &'static str>
    where
        T: IntoIterator<Item = String>,
    {
        let mut args = args.into_iter();
        let programname = args.next().unwrap();
        let programname = programname.rsplit('/').next().unwrap();
        let mut action = Action::from_program_name(programname);
        let mut filename = String::new();

        for argument in args {
            if argument.starts_with("--") {
                let mut argument_parts = argument.splitn(2, '=');
                let key = argument_parts.next().unwrap();
                let value = argument_parts.next().unwrap_or("");

                match key {
                    "--action" => action = Action::from(value),
                    _ => {},
                }
            } else {
                filename = argument
            }
        }

        if filename == "" {
            Err("No filename was given in arguments")
        } else {
            Ok(Config { filename, action })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_config_empty_args() {
        let args = Vec::new();
        let _config = Config::parse(args.into_iter());
    }

    #[test]
    fn test_config_only_programname_in_args() {
        let args = vec![String::from("run-mailcap-rs")];
        let config = Config::parse(args.into_iter());
        config.unwrap_err();
    }

    #[test]
    fn test_config_filename_in_args() {
        let args = vec![
            String::from("run-mailcap-rs"),
            String::from("test.txt"),
        ];
        let config = Config::parse(args.into_iter()).unwrap();

        assert_eq!(config.filename, "test.txt");
        assert_eq!(config.action, Action::View);
    }

    #[test]
    fn test_config_filename_and_action_in_args() {
        let args = vec![
            String::from("run-mailcap-rs"),
            String::from("--action=edit"),
            String::from("test.txt"),
        ];
        let config = Config::parse(args.into_iter()).unwrap();

        assert_eq!(config.filename, "test.txt");
        assert_eq!(config.action, Action::Edit);
    }

    #[test]
    fn test_config_action_from_programname() {
        let args = vec![
            String::from("compose"),
            String::from("test.txt"),
        ];
        let config = Config::parse(args.into_iter()).unwrap();

        assert_eq!(config.filename, "test.txt");
        assert_eq!(config.action, Action::Compose);
    }

    #[test]
    fn test_config_action_from_programname_fullpatch() {
        let args = vec![
            String::from("/usr/bin/compose"),
            String::from("test.txt"),
        ];
        let config = Config::parse(args.into_iter()).unwrap();

        assert_eq!(config.filename, "test.txt");
        assert_eq!(config.action, Action::Compose);
    }

    #[test]
    fn test_config_action_from_args_override_programname() {
        let args = vec![
            String::from("compose"),
            String::from("--action=edit"),
            String::from("test.txt"),
        ];
        let config = Config::parse(args.into_iter()).unwrap();

        assert_eq!(config.filename, "test.txt");
        assert_eq!(config.action, Action::Edit);
    }
}


