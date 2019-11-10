use indicatif::ProgressBar;

static TEMPLATE: &'static str = "{msg} {spinner:.green} {percent}% [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}";

pub fn create_progress_bar(msg: &str, length: Option<u64>) -> ProgressBar {
    ProgressBar::new(123)
}