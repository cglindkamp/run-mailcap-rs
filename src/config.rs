use regex::Regex;

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
            "see" => Action::View,
            "cat" => Action::Cat,
            "edit" => Action::Edit,
            "change" => Action::Edit,
            "compose" => Action::Compose,
            "create" => Action::Compose,
            "print" => Action::Print,
            _ => Action::View,
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub mimetype: String,
    pub mimetype_source: String,
    pub action: Action,
    pub xtermcmd: String,
    pub pager: String,
    pub running_in_x: bool,
    pub debug: bool,
    pub nopager: bool,
    pub norun: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            filename: String::new(),
            mimetype: String::new(),
            mimetype_source: String::new(),
            action: Action::View,
            xtermcmd: String::from("xterm"),
            pager: String::from("less"),
            running_in_x: false,
            debug: false,
            nopager: false,
            norun: false,
        }
    }
}

impl Config {
    pub fn parse<IA, IE>(args: IA, envvars: IE) -> Result<Config, &'static str>
    where
        IA: IntoIterator<Item = String>,
        IE: IntoIterator<Item = (String, String)>,
    {
        let mut args = args.into_iter();
        let programname = args.next().unwrap();
        let programname = programname.rsplit('/').next().unwrap();
        let mut config: Config = Config {
            action: Action::from(programname),
            ..Default::default()
        };

        for (key, value) in envvars {
            match key.as_ref() {
                "PAGER" => config.pager = value,
                "XTERMCMD" => config.xtermcmd = value,
                "DISPLAY" => config.running_in_x = true,
                _ => {},
            }
        };
        for argument in args {
            if argument.starts_with("--") {
                let mut argument_parts = argument.splitn(2, '=');
                let key = argument_parts.next().unwrap();
                let value = argument_parts.next().unwrap_or("");

                match key {
                    "--action" => config.action = Action::from(value),
                    "--debug" => config.debug = true,
                    "--nopager" => config.nopager = true,
                    "--norun" => config.norun = true,
                    _ => {},
                }
            } else {
                let re = Regex::new(r"^(?P<mimetype>[^/:]+/[^/:]+):(?P<filename>.*)").unwrap();
                if let Some(m) = re.captures(&argument) {
                    config.filename = m["filename"].to_string();
                    config.mimetype = m["mimetype"].to_string();
                }
                if config.filename == "" {
                    config.filename = argument;
                }
            }
        }

        if config.filename == "" {
            Err("No filename was given in arguments")
        } else {
            Ok(config)
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
        let env = Vec::new();
        let _config = Config::parse(args, env);
    }

    #[test]
    fn test_config_only_programname_in_args() {
        let args = vec![String::from("run-mailcap-rs")];
        let env = Vec::new();
        let config = Config::parse(args, env);
        config.unwrap_err();
    }

    #[test]
    fn test_config_defaults() {
        let args = vec![
            String::from("run-mailcap-rs"),
            String::from("test.txt"),
        ];
        let env = Vec::new();
        let config = Config::parse(args, env).unwrap();

        assert_eq!(config.filename, "test.txt");
        assert_eq!(config.action, Action::View);
        assert_eq!(config.xtermcmd, "xterm");
        assert_eq!(config.pager, "less");
        assert_eq!(config.running_in_x, false);
    }

    #[test]
    fn test_config_filename_and_action_in_args() {
        let args = vec![
            String::from("run-mailcap-rs"),
            String::from("--action=edit"),
            String::from("test.txt"),
        ];
        let env = Vec::new();
        let config = Config::parse(args, env).unwrap();

        assert_eq!(config.filename, "test.txt");
        assert_eq!(config.action, Action::Edit);
    }

    #[test]
    fn test_config_action_from_programname() {
        let args = vec![
            String::from("compose"),
            String::from("test.txt"),
        ];
        let env = Vec::new();
        let config = Config::parse(args, env).unwrap();

        assert_eq!(config.filename, "test.txt");
        assert_eq!(config.action, Action::Compose);
    }

    #[test]
    fn test_config_action_from_programname_fullpatch() {
        let args = vec![
            String::from("/usr/bin/compose"),
            String::from("test.txt"),
        ];
        let env = Vec::new();
        let config = Config::parse(args, env).unwrap();

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
        let env = Vec::new();
        let config = Config::parse(args, env).unwrap();

        assert_eq!(config.filename, "test.txt");
        assert_eq!(config.action, Action::Edit);
    }

    #[test]
    fn test_config_xtermcmd_from_env() {
        let args = vec![
            String::from("run-mailcap-rs"),
            String::from("test.txt"),
        ];
        let env = vec![
            (String::from("XTERMCMD"), String::from("urxvt")),
        ];
        let config = Config::parse(args, env).unwrap();

        assert_eq!(config.xtermcmd, "urxvt");
    }

    #[test]
    fn test_config_running_in_x_from_env() {
        let args = vec![
            String::from("run-mailcap-rs"),
            String::from("test.txt"),
        ];
        let env = vec![
            (String::from("DISPLAY"), String::from(":0")),
        ];
        let config = Config::parse(args, env).unwrap();

        assert_eq!(config.running_in_x, true);
    }

    #[test]
    fn test_config_pager_from_env() {
        let args = vec![
            String::from("run-mailcap-rs"),
            String::from("test.txt"),
        ];
        let env = vec![
            (String::from("PAGER"), String::from("more")),
        ];
        let config = Config::parse(args, env).unwrap();

        assert_eq!(config.pager, "more");
    }

    #[test]
    fn test_config_mimetype_from_args() {
        let args = vec![
            String::from("run-mailcap-rs"),
            String::from("text/plain:test.xml"),
        ];
        let env = Vec::new();

        let config = Config::parse(args, env).unwrap();

        assert_eq!(config.mimetype, "text/plain");
    }

    #[test]
    fn test_config_colon_in_filename() {
        let args = vec![
            String::from("run-mailcap-rs"),
            String::from("test:foo.txt"),
        ];
        let env = Vec::new();

        let config = Config::parse(args, env).unwrap();

        assert_eq!(config.filename, "test:foo.txt");
    }
}


