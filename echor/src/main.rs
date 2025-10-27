
fn main() {
    let matches = clap::App::new("echor")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust echo")
        .arg(
            clap::Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            clap::Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();
    
    // println!("{:#?}", _matches);
    
    let omit_newline = matches.is_present("omit_newline");
    // println!("omit_newline: {}", omit_newline);
    
    let texts = matches.values_of_lossy("text").unwrap();
    // println!("{:?}", texts);
    
    print!("{}{}", texts.join(" "), if omit_newline { "" } else { "\n" });
}
