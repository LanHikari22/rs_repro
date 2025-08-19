/**
 * This checks behavior between slugify and github_slugger for generating markdown heading links.
 * 
 * It relates to [obsidian-export #370](https://github.com/zoni/obsidian-export/issues/370).
 * 
 * Notes for this can be found [here](https://github.com/LanHikari22/lan-setup-notes/blob/webview/lan/topics/tooling/obsidian/tasks/2025/004%20Fix%20obsidian%20export%20to%20support%20internal%20links.md#36-test-rust-github-slugger-for-github-heading-dot-omission-compliance).
 * 
 * You can run this with
 * 
 * ```sh
 * git clone https://github.com/LanHikari22/rs_repro.git && cd rs_repro && cargo run --features "repro006"
 * ```
 */

use slugify::slugify;
use github_slugger::{self, Slugger};

fn print_comparison_output_for(s: &str, mut_gh_slugger: &mut Slugger) {
    let slugify_output = slugify!(s);
    let github_slugger_output = mut_gh_slugger.slug(s);

    println!("\n\nFor input {s},\n");

    println!("Service ------ | Output");
    println!("slugify        | {slugify_output}");
    println!("github_slugger | {github_slugger_output}");
}


pub fn main() {

    let mut mut_gh_slugger = github_slugger::Slugger::default();

    print_comparison_output_for("A.B.C.D", &mut mut_gh_slugger);
    print_comparison_output_for("1.2.3.4", &mut mut_gh_slugger);
    print_comparison_output_for("this-or-that", &mut mut_gh_slugger);
    print_comparison_output_for("this--or-that", &mut mut_gh_slugger);
    print_comparison_output_for("# Some headings", &mut mut_gh_slugger);
    print_comparison_output_for("repeat", &mut mut_gh_slugger);
    print_comparison_output_for("repeat", &mut mut_gh_slugger);
    print_comparison_output_for("repeat", &mut mut_gh_slugger);
}