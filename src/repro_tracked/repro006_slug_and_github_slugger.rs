use github_slugger::{self, Slugger};
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
use slug::slugify;
use std::cmp::max;

const COL_INPUT: &'static str = "input";
const COL_OUTPUT_SLUG: &'static str = "slug";
const COL_OUTPUT_GITHUB_SLUGGER: &'static str = "github_slugger";

fn compute_comparison_outputs(input: &str, mut_gh_slugger: &mut Slugger) -> (String, String) {
    (slugify(input), mut_gh_slugger.slug(input))
}

fn get_aligned_values(max_len_and_values: &[(usize, String)], filler: &str) -> Vec<String> {
    max_len_and_values
        .iter()
        .map(|(max_len, value)| {
            format!(
                "{value}{}",
                filler.repeat(max_len.saturating_sub(value.len()))
            )
        })
        .collect::<Vec<_>>()
}

fn get_aligned_spaces(max_len_and_values: &[(usize, String)], filler: &str) -> Vec<String> {
    max_len_and_values
        .iter()
        .map(|(max_len, value)| filler.repeat(max(*max_len, value.len())))
        .collect::<Vec<_>>()
}

fn print_table_header(
    max_input_len: usize,
    max_output_slug_len: usize,
    max_output_github_slugger_len: usize,
) {
    let max_len_and_columns = [
        (max_input_len, COL_INPUT.to_string()),
        (max_output_slug_len, COL_OUTPUT_SLUG.to_string()),
        (
            max_output_github_slugger_len,
            COL_OUTPUT_GITHUB_SLUGGER.to_string(),
        ),
    ];

    let aligned_columns = get_aligned_values(&max_len_and_columns, " ");
    let aligned_spaces = get_aligned_spaces(&max_len_and_columns, "-");

    println!(
        "| {} | {} | {} |",
        aligned_columns[0], aligned_columns[1], aligned_columns[2],
    );

    println!(
        "| {} | {} | {} |",
        aligned_spaces[0], aligned_spaces[1], aligned_spaces[2],
    );
}

fn print_comparison_row(
    input: &str,
    output_slug: String,
    output_github_slugger: String,
    max_input_len: usize,
    max_output_slug_len: usize,
    max_output_github_slugger_len: usize,
) {
    let max_len_and_values = [
        (max_input_len, input.to_string()),
        (max_output_slug_len, output_slug),
        (max_output_github_slugger_len, output_github_slugger),
    ];

    let aligned_values = get_aligned_values(&max_len_and_values, " ");

    println!(
        "| {} | {} | {} |",
        aligned_values[0], aligned_values[1], aligned_values[2],
    );
}

pub fn main() {
    let mut mut_gh_slugger = github_slugger::Slugger::default();

    let inputs = vec![
        "A.B.C.D",
        "1.2.3.4",
        "this-or-that",
        "this--or-that",
        "# Some heading",
        "repeat",
        "repeat",
        "repeat",
    ];

    let outputs = {
        inputs
            .iter()
            .map(|input| compute_comparison_outputs(input, &mut mut_gh_slugger))
            .collect::<Vec<_>>()
    };

    let max_input_len = {
        max(
            COL_INPUT.len(),
            inputs //_
                .iter()
                .map(|input| input.len())
                .max()
                .unwrap(),
        )
    };

    let max_output_slug_len = {
        max(
            COL_OUTPUT_SLUG.len(),
            outputs
                .iter()
                .map(|(output_slug, _)| output_slug.len())
                .max()
                .unwrap(),
        )
    };

    let max_output_github_slugger_len = {
        max(
            COL_OUTPUT_GITHUB_SLUGGER.len(),
            outputs
                .iter()
                .map(|(_, output_github_slugger)| output_github_slugger.len())
                .max()
                .unwrap(),
        )
    };

    print_table_header(
        max_input_len,
        max_output_slug_len,
        max_output_github_slugger_len,
    );

    outputs //_
        .into_iter()
        .zip(inputs.iter())
        .for_each(|((output_slug, output_github_slugger), input)| {
            print_comparison_row(
                input,
                output_slug,
                output_github_slugger,
                max_input_len,
                max_output_slug_len,
                max_output_github_slugger_len,
            );
        });
}
