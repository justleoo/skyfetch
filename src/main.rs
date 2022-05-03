mod colors;
mod utils;

use colors::*;
use columns::Columns;
use std::env::var;
use systemstat::{Duration, Platform, System};

const ASCII: &str = "
   __   _
 _(  )_( )_
(_   _    _)
  (_) (__)";

fn main() {
    let sys = System::new();

    // get wm
    let wm = utils::get_wm().unwrap_or_else(|| String::from("unknown"));

    // get terminal
    let term = var("TERM").unwrap_or_else(|_| String::from("unknown"));

    // get shell
    let shell = utils::get_shell().unwrap_or_else(|| String::from("unknown"));

    // get uptime
    let uptime = sys.uptime().unwrap_or_else(|_| Duration::default());

    // format fetch text
    let fetch_text = Columns::from(vec![
        format!("{BLUE}{ASCII}").split('\n').collect::<Vec<&str>>(),
        vec![
            &format!(
                "     {WHITE}{}{RED}@{RESET}{}{BLUE}",
                whoami::username(),
                whoami::hostname()
            ),
            &format!("{CYAN}os {WHITE}  ~ {CYAN}{}{BLUE}", whoami::distro()),
            &format!("{YELLOW}upt {WHITE} ~ {YELLOW}{uptime:?}{BLUE}"),
            &format!("{GREEN}wm {WHITE}  ~ {GREEN}{wm}{BLUE}"),
            &format!("{MAGENTA}term{WHITE} ~ {MAGENTA}{term}{BLUE}"),
            &format!("{YELLOW_BRIGHT}sh {WHITE}  ~ {YELLOW_BRIGHT}{shell}{BLUE}"),
            &format!("{RED}● {YELLOW}● {CYAN}● {BLUE}● {WHITE}●"),
        ],
    ])
    .set_tabsize(15)
    .make_columns();

    println!("{fetch_text}");
}
