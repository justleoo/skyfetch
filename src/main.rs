extern crate systemstat;
mod colors;
use std::env;
use systemstat::{System, Platform};

fn main() {
    let sys = System::new();

    // get wm
    let wm = env::var("DESKTOP_SESSION").unwrap_or_else(|_| {
        env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "!!UNKNOWN!!".to_owned())
    });

    // get terminal
    let term = env::var("TERM").unwrap_or_else(|_| "!!UNKNOWN!!".to_owned());

    // get shell
    let shell = match env::var("SHELL") {
        Ok(var) => {
            let split: Vec<_> = var.split('/').collect();
            match split.last() {
                Some(sh) => sh.to_string(),
                    _ => "unknown".to_string(),
            }
        }
         _ => "unknown".to_string(),
    };

    // username and hostname
    println!("               {white}{}{red}{}{white}{}", whoami::username(), "@", whoami::hostname(), red = colors::red, white = colors::white);

    // os
    println!("{blue}   __   _      {cyan}{}{white}{}{cyan}{}", "os ", "  ~ ", whoami::distro(), blue = colors::blue, cyan = colors::cyan, white = colors::white);

    // uptime
    match sys.uptime() {
        Ok(uptime) => println!("{blue} _(  )_( )_    {yellow}upt {white} ~ {yellow}{:?}", uptime, white = colors::white, yellow = colors::yellow, blue = colors::blue),
        Err(x) => println!("{yellow}uptime error {white}~ {yellow}{}", x, white = colors::white, yellow = colors::yellow)
    }

    // wm
    println!("{blue}(_   _    _)   {green}{}{white}{}{green}{}", "wm ", "  ~ ", wm, blue = colors::blue, green = colors::green, white = colors::white);
    
    // terminal
    println!("{blue}  (_) (__)     {magenta}{}{white}{}{magenta}{}", "term", " ~ ", term, magenta = colors::magenta, white = colors::white, blue = colors::blue);

    // shell
    println!("               {yellow_bright}{}{white}{}{yellow_bright}{}", "sh ", "  ~ ", shell, yellow_bright = colors::yellow_bright, white = colors::white);
    // decoration
    println!("               {red}{} {yellow}{} {cyan}{} {blue}{} {white}{}", "●", "●", "●", "●", "●", red = colors::red, blue = colors::blue, yellow = colors::yellow, cyan = colors::cyan, white = colors::white);
}
