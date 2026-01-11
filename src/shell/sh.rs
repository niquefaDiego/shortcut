const NAME: &str = "Bourne Shell";
const BASH_FUNCTION_FILE: &str = include_str!("./script/posix.sh");
let bashrc_dir = std::path::PathBuf::from("~/.profile");
