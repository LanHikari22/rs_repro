use slugify::slugify;
use github_slugger;

fn print_comparison_output_for(s: &str) {
    let slugify_output = slugify!(s);
    let github_slugger_output = github_slugger::slug(s);

    println!("\n\nFor input {s},\n");

    println!("Service ------ | Output");
    println!("slugify        | {slugify_output}");
    println!("github_slugger | {github_slugger_output}");
}


pub fn main() {
    print_comparison_output_for("A.B.C.D");
    print_comparison_output_for("1.2.3.4");
    print_comparison_output_for("this-or-that");
    print_comparison_output_for("this--or-that");
    print_comparison_output_for("# Some headings");
}