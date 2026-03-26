pub mod data;
pub mod runner;

use std::path::PathBuf;

pub use runner::{BenchmarkRunner, CliTool};

pub fn run(category: Option<String>, cli: CliTool) -> Result<(), Box<dyn std::error::Error>> {
    let prompts_dir = PathBuf::from("benchmark/prompts");
    let output_dir = PathBuf::from("benchmark/results");

    let categories = if let Some(cat) = category {
        vec![data::PromptCategory::from_yaml(
            &prompts_dir.join(format!("{}.yaml", cat)),
        )?]
    } else {
        data::PromptCategory::load_all(&prompts_dir)?
    };

    let runner = BenchmarkRunner::new(output_dir, cli);

    for cat in &categories {
        println!("\n=== Category: {} ===\n", cat.name);
        for task in &cat.tasks {
            println!("Running: {}", task.id);

            let with_leankg = runner.run_with_leankg(&task.prompt);
            let without_leankg = runner.run_without_leankg(&task.prompt);

            let overhead = with_leankg.overhead(&without_leankg);

            println!(
                "  With LeanKG: {} tokens (input: {}, cached: {})",
                with_leankg.total_tokens, with_leankg.input_tokens, with_leankg.cached_tokens
            );
            println!(
                "  Without: {} tokens (input: {}, cached: {})",
                without_leankg.total_tokens,
                without_leankg.input_tokens,
                without_leankg.cached_tokens
            );
            println!("  Overhead: {} tokens\n", overhead.token_delta);

            let _ = runner.save_comparison(&with_leankg, &without_leankg, &task.id);
        }
    }

    Ok(())
}
