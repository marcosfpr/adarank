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

    // Get ohsumed location from the root project path /examples/ohsumed
    let corpus = std::path::Path::new("benchmarks/OHSUMED").join("Data/All/OHSUMED.txt");

    if corpus.exists() {
        log::info!("Loading corpus from {}", corpus.display());
        let ohsumed_dataset: DataSet = ltrs::loader::svmlight::SVMLight::load(
            corpus.to_str().unwrap()
        )
        .unwrap_or_else(|_| {
            panic!(
                "Could not load ohsumed dataset located at {}/Data/All/OHSUMED.txt",
                corpus.display()
            )
        });
    
        let mut adarank = AdaRank::new(ohsumed_dataset, Box::new(MAP), 50, 3, 0.003, None, None);
    
        adarank.fit().unwrap();
    
        log::debug!("Finished fitting");
    } else {
        log::error!("Corpus not found at {}", corpus.display());
        std::process::exit(1);
    }

}
