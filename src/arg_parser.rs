use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "LIFARS LLC")]
pub struct Opts {
    #[clap(
    short = "c",
    long = "computer",
    default_value = "127.0.0.1",
    help = "Remote computer address/name. Not setting this equals 127.0.0.1"
    )]
    pub computer: String,

    #[clap(
    short = "u",
    long = "user",
    help = "Remote user name"
    )]
    pub user: String,

    #[clap(
    short = "p",
    long = "password",
    help = "Remote user name"
    )]
    pub password: String,
    #[clap(
    short = "o",
    long = "output",
    default_value = "program-output",
    help = "Remote user password"
    )]
    pub store_directory: String,

    #[clap(
    short = "e",
    long = "commands",
    help = "Optional: File with custom commands to execute on remote computer"
    )]
    pub custom_command_path: Option<String>,

    #[clap(
    short = "s",
    long = "search",
    help = "Optional: File with files names to be searched on remote computer. \
    Supports also `*` and `?` wildcards."
    )]
    pub search_files_path: Option<String>,

    #[clap(short = "a", long = "all")]
    pub all: bool,
    #[clap(long = "wmi")]
    pub wmi: bool,
    #[clap(long = "rdp")]
    pub rdp: bool,
    #[clap(long = "psexec")]
    pub psexec: bool,
    #[clap(long = "psrem")]
    pub psremote: bool,

    #[clap(short = "f", long = "fast")]
    pub fast_mode: bool,
}
