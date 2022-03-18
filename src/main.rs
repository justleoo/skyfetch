extern crate systemstat;
#[warn(unused_imports)]
mod colors;
use systemstat::{System, Platform, saturating_sub_bytes};

fn main() {
    let sys = System::new();

    // username and hostname
    println!("               {white}{}{red}{}{white}{}", whoami::username(), "@", whoami::hostname(), red = colors::red, white = colors::white);

    // os
    println!("{blue}   __   _      {cyan}{}{white}{}{cyan}{}", "os ", "~ ", whoami::distro(), blue = colors::blue, cyan = colors::cyan, white = colors::white);

    // uptime
    match sys.uptime() {
        Ok(uptime) => println!("{blue} _(  )_( )_    {yellow}uptime {white}~ {yellow}{:?}", uptime, white = colors::white, yellow = colors::yellow, blue = colors::blue),
        Err(x) => println!("{yellow}uptime error {white}~ {yellow}{}", x, white = colors::white, yellow = colors::yellow)
    }

    // plataform
    println!("{blue}(_   _    _)   {red}{}{white}{}{red}{}", "plataform ", "~ ", whoami::platform(), blue = colors::blue, red = colors::red, white = colors::white);

    // 
    println!("{blue}  (_) (__)", blue = colors::blue);

    // decoration
    println!("               {red}{} {yellow}{} {cyan}{} {blue}{} {white}{}", "●", "●", "●", "●", "●", red = colors::red, blue = colors::blue, yellow = colors::yellow, cyan = colors::cyan, white = colors::white);
}
