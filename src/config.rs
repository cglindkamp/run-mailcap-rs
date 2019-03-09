#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub action: String,
}

impl Config {
    pub fn parse<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>,
    {
        let programname = args.next().unwrap();
        let programname = programname.rsplit('/').next().unwrap();
        let mut action = match programname {
            "view" => String::from("view"),
            "view-rs" => String::from("view"),
            "see" => String::from("view"),
            "see-rs" => String::from("view"),
            "cat" => String::from("cat"),
            "cat-rs" => String::from("cat"),
            "edit" => String::from("edit"),
            "edit-rs" => String::from("edit"),
            "change" => String::from("edit"),
            "change-rs" => String::from("edit"),
            "compose" => String::from("compose"),
            "compose-rs" => String::from("compose"),
            "create" => String::from("compose"),
            "create-rs" => String::from("compose"),
            "print" => String::from("print"),
            "print-rs" => String::from("print"),
            _ => String::from("view"),
        };
        let mut filename = String::new();

        for argument in args {
            if argument.starts_with("--") {
                let mut argument_parts = argument.splitn(2, "=");
                let key = argument_parts.next().unwrap();
                let value = argument_parts.next().unwrap_or("");

                match key {
                    "--action" => action = String::from(value),
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
        assert_eq!(config.action, "view");
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
        assert_eq!(config.action, "edit");
    }

    #[test]
    fn test_config_action_from_programname() {
        let args = vec![
            String::from("compose"),
            String::from("test.txt"),
        ];
        let config = Config::parse(args.into_iter()).unwrap();

        assert_eq!(config.filename, "test.txt");
        assert_eq!(config.action, "compose");
    }

    #[test]
    fn test_config_action_from_programname_fullpatch() {
        let args = vec![
            String::from("/usr/bin/compose"),
            String::from("test.txt"),
        ];
        let config = Config::parse(args.into_iter()).unwrap();

        assert_eq!(config.filename, "test.txt");
        assert_eq!(config.action, "compose");
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
        assert_eq!(config.action, "edit");
    }
}


