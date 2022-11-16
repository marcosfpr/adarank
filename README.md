<!-- PROJECT LOGO -->
<br />
<p align="center">
  <h3 align="center">lt.rs</h3>

  <p align="center">
    Learning to Rank for Rustaceans [Early stages of development]
    <br />
    <a href=""><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/marcosfpr/lt.rs/issues">Report Bug</a>
    ·
    <a href="https://github.com/marcosfpr/lt.rs/issues">Request Feature</a>
  </p>
</p>

### 🌟 Machine Learning for Ranking problems.

The problem of ranking documents in a given corpus is central to Information Retrieval. Although this problem has more notoriety in search engines, those ranking algorithms can be used in different solutions such as collaborative filtering, question answering, multimedia retrieval, text summarization, and online advertising.

Recently, regarding the massive amount of data available for training, it's become possible to leverage existing Machine Learning (ML) technologies to build more effective ranking models. Using supervised ML techniques to solve ranking problems is called Learning to Rank (LTR). 


<br/>

[![Build Status](https://img.shields.io/github/workflow/status/marcosfpr/ltrs/Unit%20tests)](https://img.shields.io/github/workflow/status/marcosfpr/ltrs/Unit%20tests)
[![Version](https://img.shields.io/crates/v/ltrs)](https://crates.io/crates/ltrs)
[![Docs](https://img.shields.io/docsrs/ltrs)](https://docs.rs/ltrs)
[![Coverage](https://img.shields.io/codecov/c/github/marcosfpr/ltrs)](https://img.shields.io/codecov/c/github/marcosfpr/ltrs)
[![Issues](https://img.shields.io/github/issues/marcosfpr/ltrs)](https://img.shields.io/github/issues/marcosfpr/ltrs)

<br/>


### ✨ Features
Based on the very well-known library `RankLib`, lt.rs provides LTR models and a solid interface so that you can implement your own model on
the platform.

- ⚡️ **SVMLight support**
- ⚡️ **AdaRank boosting method**
- ⚡️ **Different relevance metrics to tune the models**
- ⚡️ **Future: CLI application**

### Performance
ltrs can provide the ability to fit an LTR model based on a given corpus. Currently, we have a good performance in our models but still have some limitations described below.

Performance is an absolute requirement for those kinds of applications, especially for large datasets. So that's definitively one thing that I'm working towards and will be present here soon...

### 💔 Limitations
Although ltrs provides really interesting features, it can not handle some important things by now. It has some limitations:

- ltrs does not handle large files that cannot be loaded directly into main memory;
- ltrs does not support as many LTR models as we would like to (yet)
