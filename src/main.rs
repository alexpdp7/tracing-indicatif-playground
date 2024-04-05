/// sync/rayon version of https://github.com/emersonford/tracing-indicatif/blob/main/examples/basic.rs
use std::time::Duration;

use rand::thread_rng;
use rand::Rng;
use rayon::prelude::*;
use tracing::info;
use tracing::instrument;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[instrument]
fn do_work(val: u64) -> u64 {
    let sleep_time = thread_rng().gen_range(Duration::from_millis(250)..Duration::from_millis(500));
    std::thread::sleep(sleep_time);

    info!("doing work for val: {}", val);

    let sleep_time =
        thread_rng().gen_range(Duration::from_millis(500)..Duration::from_millis(1000));
    std::thread::sleep(sleep_time);

    val + 1
}

fn main() {
    let indicatif_layer = IndicatifLayer::new();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(indicatif_layer.get_stderr_writer()))
        .with(indicatif_layer)
        .init();

    let res: u64 = (0..20)
        .into_par_iter()
        .map(|val| do_work(val))
        .collect::<Vec<u64>>()
        .into_iter()
        .sum();

    println!("final result: {}", res);
}
