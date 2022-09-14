use crate::utils::color::*;

pub fn usage(exec: &str) {
    println!(
"{purp}Usage{green}: {exec} {purp}[{green}flags{purp}]{green} {purp}[{green}path{purp}]{green}
{purp}FLAGS{green}:
    -h, --help    =>    Shows this screen.

{ylw}Made by MCorange :){rs}", exec=exec,
// red=RED,
green=GREEN,
rs=RESET,
purp=VIOLET,
// ul=UNDERLINE
ylw=YELLOW
)
}