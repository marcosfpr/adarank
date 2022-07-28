use ltrs::{
    ensemble::adarank::AdaRank,
    eval::{map::MAP, precision::Precision},
    learner::Learner,
    loader::LtrFormat,
    utils::logging::init_logger,
    DataSet,
};

fn main() {
    init_logger();

    // Get ohsumed location from environment variable OHSUMED
    let corpus_path = std::env::var("OHSUMED").unwrap_or_else(|_| {
        panic!("Please set the environment variable OHSUMED to the location of the ohsumed dataset")
    });

    let ohsumed_dataset: DataSet = ltrs::loader::svmlight::SVMLight::load(
        format!("{}/Data/All/OHSUMED.txt", corpus_path).as_str(),
    )
    .unwrap_or_else(|_| {
        panic!(
            "Could not load ohsumed dataset located at {}/Data/All/OHSUMED.txt",
            corpus_path
        )
    });

    let mut adarank = AdaRank::new(ohsumed_dataset, Box::new(MAP), 50, 3, 0.003, None, None);

    adarank.fit().unwrap();

    log::debug!("Finished fitting");
}
