pub mod data;

use std::path::PathBuf;
use std::process::Command;

pub struct BenchmarkRunner {
    opencode_path: PathBuf,
    output_dir: PathBuf,
}

impl BenchmarkRunner {
    pub fn new(output_dir: PathBuf) -> Self {
        Self {
            opencode_path: PathBuf::from("opencode"),
            output_dir,
        }
    }

    pub fn run_with_leankg(&self, prompt: &str) -> data::BenchmarkResult {
        let output = Command::new(&self.opencode_path)
            .arg(prompt)
            .output()
            .expect("Failed to execute opencode");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        self.parse_opencode_output(&stdout, &stderr)
    }

    pub fn run_without_leankg(&self, prompt: &str) -> data::BenchmarkResult {
        let output = Command::new(&self.opencode_path)
            .arg(prompt)
            .env("LEANKG_DISABLED", "1")
            .arg(prompt)
            .output()
            .expect("Failed to execute opencode");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        self.parse_opencode_output(&stdout, &stderr)
    }

    fn parse_opencode_output(&self, stdout: &str, stderr: &str) -> data::BenchmarkResult {
        let combined = format!("{}\n{}", stdout, stderr);

        let mut tokens = 0u32;
        let mut percent = 0f32;
        let mut time = 0f32;

        if let Some(token_match) = regex::Regex::new(r"(\d{1,3}(?:,\d{3})*)\s+(\d+)%")
            .unwrap()
            .captures(&combined)
        {
            tokens = token_match[1].replace(',', "").parse().unwrap_or(0);
            percent = token_match[2].parse().unwrap_or(0f32);
        }

        if let Some(time_match) = regex::Regex::new(r"Build.*?(\d+\.?\d*)s")
            .unwrap()
            .captures(&combined)
        {
            time = time_match[1].parse().unwrap_or(0f32);
        }

        data::BenchmarkResult {
            total_tokens: tokens,
            token_percent: percent,
            build_time_seconds: time,
            success: true,
        }
    }

    pub fn save_comparison(
        &self,
        with_leankg: &data::BenchmarkResult,
        without_leankg: &data::BenchmarkResult,
        name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let overhead = with_leankg.overhead(without_leankg);

        let comparison = serde_json::json!({
            "task": name,
            "with_leankg": with_leankg,
            "without_leankg": without_leankg,
            "overhead": overhead,
        });

        let json_path = self.output_dir.join(format!("{}-comparison.json", name));
        std::fs::write(&json_path, serde_json::to_string_pretty(&comparison)?)?;

        let md_path = self.output_dir.join(format!("{}-comparison.md", name));
        let md = format!(
            "# Benchmark Comparison: {}\n\n## With LeanKG\n- Tokens: {}\n- Token %: {}%\n- Time: {}s\n\n## Without LeanKG\n- Tokens: {}\n- Token %: {}%\n- Time: {}s\n\n## Overhead\n- Token Delta: {}\n- Token Delta %: {}%\n- Time Delta: {}s\n",
            name,
            with_leankg.total_tokens, with_leankg.token_percent, with_leankg.build_time_seconds,
            without_leankg.total_tokens, without_leankg.token_percent, without_leankg.build_time_seconds,
            overhead.token_delta, overhead.token_delta_percent, overhead.time_delta
        );
        std::fs::write(&md_path, md)?;

        Ok(())
    }
}

pub fn run(category: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let prompts_dir = PathBuf::from("benchmark/prompts");
    let output_dir = PathBuf::from("benchmark/results");

    let categories = if let Some(cat) = category {
        vec![data::PromptCategory::from_yaml(
            &prompts_dir.join(format!("{}.yaml", cat)),
        )?]
    } else {
        data::PromptCategory::load_all(&prompts_dir)?
    };

    let runner = BenchmarkRunner::new(output_dir);

    for cat in &categories {
        println!("\n=== Category: {} ===\n", cat.name);
        for task in &cat.tasks {
            println!("Running: {}", task.id);

            let with_leankg = runner.run_with_leankg(&task.prompt);
            let without_leankg = runner.run_without_leankg(&task.prompt);

            let overhead = with_leankg.overhead(&without_leankg);

            println!(
                "  With LeanKG: {} tokens ({}%)",
                with_leankg.total_tokens, with_leankg.token_percent
            );
            println!(
                "  Without: {} tokens ({}%)",
                without_leankg.total_tokens, without_leankg.token_percent
            );
            println!(
                "  Overhead: {} tokens ({}%)",
                overhead.token_delta, overhead.token_delta_percent
            );

            let _ = runner.save_comparison(&with_leankg, &without_leankg, &task.id);
        }
    }

    Ok(())
}
