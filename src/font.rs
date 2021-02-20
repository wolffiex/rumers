use std::cmp;
use std::iter::Iterator;

const ZERO: &str = r"
  ___
 / _ \
| | | |
| | | |
| |_| |
 \___/
";
const ONE: &str = r"
 __
/_ |
 | |
 | |
 | |
 |_|
";
const TWO: &str = r"
 ___
|__ \
   ) |
  / /
 / /_
|____|
";
const THREE: &str = r"
 ____
|___ \
  __) |
 |__ <
 ___) |
|____/
";
const FOUR: &str = r"
 _  _
| || |
| || |_
|__   _|
   | |
   |_|
";

const FIVE: &str = r"
 _____
| ____|
| |__
|___ \
 ___) |
|____/
";

const SIX: &str = r"
   __
  / /
 / /_
| '_ \
| (_) |
 \___/
";

const SEVEN: &str = r"
 ______
|____  |
    / /
   / /
  / /
 /_/
";

const EIGHT: &str = r"
  ___
 / _ \
| (_) |
 > _ <
| (_) |
 \___/
";

const NINE: &str = r"
  ___
 / _ \
| (_) |
 \__, |
   / /
  /_/
";

pub fn get_font() -> usize {
    let numerals= [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];
    let widths = numerals.iter().map(|s| max_line_width(s));
    let max_width = widths.clone().fold(0, cmp::max);

    let font = widths.zip(numerals.iter()).map(|(width, numeral)| {
        let formatted = numeral.lines().skip(1).fold("".to_owned(), |acc, line| {
            acc + line
        });
        (width, formatted)
    }).collect::<Vec<(usize, String)>>();
    println!("widths {:#?}", font);
    max_width
}

fn max_line_width(s:&str ) -> usize {
    s.lines().map(|l| l.len()).fold(0, cmp::max)
}
