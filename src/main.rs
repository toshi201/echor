use clap::{arg, Command};

fn main() {
    let matches = Command::new("Echo")
    .version("0.1.0")
    .author("Ken you")
    .about("Rust echo")
    .arg(
        arg!(text: <TEXT>)
        .required(true)
        .value_delimiter(' ')
        .num_args(1..)
        .help("input text")
    
    )
    .arg(
        arg!(omit_newline: --omit)
        .short('n')
        .help("Do not print newline")
    )
    .get_matches();

    let text_vec: Vec<String> = matches.get_many::<String>("text").unwrap().cloned().collect();
    let text = text_vec.join(" ");
    let endin = if matches.get_flag("omit_newline") { "" } else { "\n" };
    print!("{}{}", text, endin);
}
