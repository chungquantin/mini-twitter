use colored::Colorize;

pub fn log_stage(stage: &'static str, title: &'static str) {
    println!(
        "{} : {}",
        format!("{}", stage).yellow(),
        format!("{}", title).bold()
    );
}
