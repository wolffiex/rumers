#[allow(dead_code)]
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

fn get_font() {
    let numerals: Vec<&str> = vec![ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];
}