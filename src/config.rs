use clap::{App, Arg, ArgMatches};

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Pass,
    TextOnly,
    PrettyPrint,
    Attributes(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub input_path: String,
    pub output_path: String,
    pub selector: String,
    pub base: Option<String>,
    pub detect_base: bool,
    pub output_format: OutputFormat,
    pub ignore_whitespace: bool,
    pub remove_nodes: Option<Vec<String>>,
}

impl Config {
    fn from_args(matches: ArgMatches) -> Option<Config> {
        let attributes = matches
            .values_of("attribute")
            .map(|values| values.map(String::from).collect());

        let remove_nodes = matches
            .values_of("remove_nodes")
            .map(|values| values.map(String::from).collect());

        let selector: String = match matches.values_of("selector") {
            Some(values) => values.collect::<Vec<&str>>().join(" "),
            None => String::from("html"),
        };

        let base = matches.value_of("base").map(|b| b.to_owned());

        let output_format = {
            if let Some(attributes) = attributes {
                OutputFormat::Attributes(attributes)
            } else if matches.is_present("pretty_print") {
                OutputFormat::PrettyPrint
            } else if !matches.is_present("text_only") {
                OutputFormat::Pass
            } else {
                OutputFormat::TextOnly
            }
        };

        Some(Config {
            input_path: String::from(matches.value_of("filename").unwrap_or("-")),
            output_path: String::from(matches.value_of("output").unwrap_or("-")),
            base,
            detect_base: matches.is_present("detect_base"),

            ignore_whitespace: matches.is_present("ignore_whitespace"),
            output_format,
            remove_nodes,
            selector,
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_path: "-".to_string(),
            output_path: "-".to_string(),
            selector: "html".to_string(),
            base: None,
            detect_base: false,
            ignore_whitespace: true,
            output_format: OutputFormat::PrettyPrint,
            remove_nodes: None,
        }
    }
}

fn get_app<'a, 'b>() -> App<'a, 'b> {
    App::new("htmlq")
        .version("0.4.0")
        .author("Michael Maclean <michael@mgdm.net>")
        .about("Runs CSS selectors on HTML")
        .arg(
            Arg::with_name("filename")
                .short("f")
                .long("filename")
                .value_name("FILE")
                .help("The input file. Defaults to stdin")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("The output file. Defaults to stdout")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("pretty_print")
                .short("p")
                .long("pretty")
                .help("Pretty-print the serialised output"),
        )
        .arg(
            Arg::with_name("text_only")
                .short("t")
                .long("text")
                .help("Output only the contents of text nodes inside selected elements"),
        )
        .arg(
            Arg::with_name("ignore_whitespace")
                .short("w")
                .long("ignore-whitespace")
                .help("When printing text nodes, ignore those that consist entirely of whitespace"),
        )
        .arg(
            Arg::with_name("attribute")
                .short("a")
                .long("attribute")
                .takes_value(true)
                .help("Only return this attribute (if present) from selected elements"),
        )
        .arg(
            Arg::with_name("base")
                .short("b")
                .long("base")
                .takes_value(true)
                .help("Use this URL as the base for links"),
        )
        .arg(
            Arg::with_name("detect_base")
                .short("B")
                .long("detect-base")
                .help("Try to detect the base URL from the <base> tag in the document. If not found, default to the value of --base, if supplied"),
        )
        .arg(
            Arg::with_name("remove_nodes")
                .long("remove-nodes")
                .short("r")
                .multiple(true)
                .number_of_values(1)
                .takes_value(true)
                .value_name("SELECTOR")
                .help("Remove nodes matching this expression before output. May be specified multiple times")
        )
        .arg(
            Arg::with_name("selector")
                .default_value("html")
                .multiple(true)
                .help("The CSS expression to select"),
        )
}

pub fn get_config() -> Config {
    let app = get_app();
    let matches = app.get_matches();
    Config::from_args(matches).unwrap_or_default()
}
