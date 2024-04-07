use tracing::{error, event, info, info_span, Level, span};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::util::SubscriberInitExt;

fn main() {
    let subscriber = tracing_subscriber::fmt().with_max_level(Level::INFO).finish();
    // tracing_subscriber::fmt().with_max_level(Level::TRACE).init();
    // LevelFilter::current().

    // tracing::collect::with_default(collector, || {
    //     info!("hello");
    // });

    tracing::subscriber::set_global_default(subscriber);
    info!("aaaaaaaaaaaaaaaaaaa");
    do_sth(123);

    // let number_of_yaks = 3;
    // info!(number_of_yaks=11, "preparing to shave yaks");
    //
    // info!("adfadfsfadsf {}", number_of_yaks);
}

fn do_sth(args: u32) {
    let span = span!(Level::TRACE, "do_sth", args);
    info!("before do_sth");
    inner();

    for i in 0..5 {
        let span = span!(Level::TRACE, "my_loop", iteration = i);
        let _enter = span.enter();
        // let n = 3;
        error!(key=123, "do_sth");
    }
}

fn inner() {
    info!("1111111111111111111111111111111111");
}