# AdaRank: a boosting algorithm for information retrieval

> [!WARNING]
> This library is not stable and not maintained. Do not use it for anything serious. 

Implementation in rust of the AdaRank algorithm for learning to rank. Abstract of the original paper:

> In this paper we address the issue of learning to rank for document retrieval. In the task, a model is automatically created with some training data and then is utilized for ranking of documents. The goodness of a model is usually evaluated with performance measures such as MAP (Mean Average Precision) and NDCG (Normalized Discounted Cumulative Gain). Ideally a learning algorithm would train a ranking model that could directly optimize the performance measures with respect to the training
> data. Existing methods, however, are only able to train ranking models by minimizing loss functions loosely related to the performance measures. For example, Ranking SVM and RankBoost train ranking models by minimizing classification errors on instance pairs. To deal with the problem, we propose a novel learning algorithm within the framework of boosting, which can minimize a loss function directly defined on the performance measures. Our algorithm, referred to as AdaRank,
> repeatedly constructs 'weak rankers' on the basis of reweighted training data and finally linearly combines the weak rankers for making ranking predictions. We prove that the training process of AdaRank is exactly that of enhancing the performance measure used. Experimental results on four benchmark datasets show that AdaRank significantly outperforms the baseline methods of BM25, Ranking SVM, and RankBoost.

# Example

An example using AdaRank to rank documents using `adarank` for the OHSUMED dataset:

```rust
use adarank::{
    ensemble::adarank::AdaRank,
    eval::map::MAP,
    learner::Learner,
    loader::{svmlight::SVMLight, LtrFormat},
    ranker::Ranker,
    DataSet,
};


fn main() {
    let corpus = std::path::Path::new("benchmarks/OHSUMED").join("Data/All/OHSUMED.txt");

    let ohsumed_dataset: DataSet =
        SVMLight::load(corpus.to_str().unwrap()).unwrap_or_else(|_| {
            panic!(
                "Could not load ohsumed dataset located at {}/Data/All/OHSUMED.txt",
                corpus.display()
            )
    });

    let mut adarank = AdaRank::new(ohsumed_dataset, Box::new(MAP), 50, 3, 0.003, None, None);

    adarank.fit().unwrap();

    let doc_label = adarank.predict(&test_sample.get(0).unwrap());
    println!(
        "Document {} has the score {:.2} for query {}",
        dp.get_description().unwrap(),
        doc_label,
        dp.get_query_id()
    );
}
```

