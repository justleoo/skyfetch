mod colors;
mod utils;
use colors::*;
use std::env;
use systemstat::{Platform, System};
fn main() {
    let sys = System::new();

    // get wm
    let wm = utils::get_wm().unwrap_or_else(|| "unknown".to_string());

    // get terminal
    let term = env::var("TERM").unwrap_or_else(|_| "unknown".to_owned());

    // get shell
    let shell = utils::get_shell().unwrap_or_else(|| "unknown".to_string());

    // username and hostname
    println!(
        "               {WHITE}{}{RED}@{RESET}{}",
        whoami::username(),
        whoami::hostname(),
    );

    // os
    println!(
        "{BLUE}   __   _      {CYAN}os {WHITE}  ~ {CYAN}{}",
        whoami::distro(),
    );

    // uptime
    if let Ok(uptime) = sys.uptime() {
        println!("{BLUE} _(  )_( )_    {YELLOW}upt {WHITE} ~ {YELLOW}{uptime:?}",)
    }

    // wm
    println!("{BLUE}(_   _    _)   {GREEN}wm {WHITE}  ~ {GREEN}{wm}");

    // terminal
    println!("{BLUE}  (_) (__)     {MAGENTA}term{WHITE} ~ {MAGENTA}{term}");

    // shell
    println!("               {YELLOW_BRIGHT}sh {WHITE}  ~ {YELLOW_BRIGHT}{shell}");

    // decoration
    println!("               {RED}● {YELLOW}● {CYAN}● {BLUE}● {WHITE}●")
}
