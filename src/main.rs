mod app;
mod ping;

fn main() {
    show_title();
    show_usage();
    if !ping::is_available() {
        !todo!()
    }
    match ping::foo() {
        Ok(_) => println!("{}", "was coolio"),
        Err(_) => println!("{}", "was NOT coolio"),
    }
}

fn show_title() {
    println!();
    println!(
        "{} {} (by {} {})",
        app::NAME,
        app::VERSION,
        app::AUTHOR,
        app::DATE
    );
    println!();
}

pub fn show_usage() {
    println!("A tiny Windows command line tool, endlessly pinging Telekom´s main DNS server.");
    println!();
    println!("Have a look at \"https://github.com/mbodm/wauzcmd\" for more information.");
    println!();
}
