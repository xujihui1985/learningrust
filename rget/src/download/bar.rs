use indicatif::{ProgressBar, ProgressStyle};

static TEMPLATE: &'static str =
    "{msg} {spinner:.green} {percent}% [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}";

pub fn create_progress_bar(msg: &str, length: Option<u64>) -> ProgressBar {
    let progbar = match length {
        Some(l) => ProgressBar::new(l),
        None => ProgressBar::new_spinner(),
    };

    progbar.set_message(msg);
    if length.is_some() {
        progbar.set_style(
            ProgressStyle::default_bar()
                .template(TEMPLATE)
                .progress_chars("=> "),
        );
    } else {
        progbar.set_style(ProgressStyle::default_spinner());
    }

    progbar
}
