use difference::{Changeset, Difference};
use owo_colors::OwoColorize;

pub fn print_diff(text1: &str, text2: &str) {
    let Changeset { diffs, .. } = Changeset::new(text1, text2, "\n");

    for diff in diffs {
        match diff {
            Difference::Same(ref x) => {
                println!("{}", x);
            }
            Difference::Add(ref x) => {
                print!("{} ", x.green());
            }
            Difference::Rem(ref x) => {
                println!("-{}", x.red());
            }
        }
    }
}
