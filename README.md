# compensated-summation

> Compensated summation algorithms for better precision.

[![crates.io](https://img.shields.io/crates/v/compensated-summation?logo=rust)](https://crates.io/crates/compensated-summation)
[![docs.rs](https://img.shields.io/docsrs/compensated-summation?logo=docsdotrs)](https://docs.rs/compensated-summation)
[![GitHub](https://img.shields.io/static/v1?label=github&message=FedericoStra/compensated-summation&color=brightgreen&logo=github)](https://github.com/FedericoStra/compensated-summation)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/FedericoStra/compensated-summation/rust.yml?logo=githubactions&logoColor=white)](https://github.com/FedericoStra/compensated-summation/actions/workflows/rust.yml)
[![Dependencies status](https://deps.rs/repo/github/FedericoStra/compensated-summation/status.svg)](https://deps.rs/repo/github/FedericoStra/compensated-summation)
[![MIT license](https://img.shields.io/crates/l/compensated-summation)](https://choosealicense.com/licenses/mit/)

This crate implements summation algorithms that significantly reduce the numerical error in the total obtained by adding a sequence of finite-precision floating-point numbers, compared to the obvious approach.

Currently it implements the `2Sum` and `Fast2Sum` from <https://en.wikipedia.org/wiki/2Sum> for exact addition and the [Kahan-Babuška](https://en.wikipedia.org/wiki/Kahan_summation_algorithm#The_algorithm) and [Kahan-Babuška-Neumaier](https://en.wikipedia.org/wiki/Kahan_summation_algorithm#Further_enhancements) algorithms for compensated summation.

Please see the [documentation](https://docs.rs/compensated-summation) for a description of the API and some usage examples.
