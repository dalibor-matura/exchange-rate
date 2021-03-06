# Exchange Rate

The best Exchange Rate path search for a given set of Exchanges, Currencies, Exchange Rates and Exchange Rate Requests. 

|Crate|Documentation|Travis CI|CodeCov|
|:---:|:-----------:|:-------:|:-----:|
|[![Crate](http://meritbadge.herokuapp.com/exchange-rate)](https://crates.io/crates/exchange-rate)|[![Documentation](https://docs.rs/exchange-rate/badge.svg)](https://docs.rs/exchange-rate)|[![Build Status](https://travis-ci.org/dalibor-matura/exchange-rate.svg?branch=master)](https://travis-ci.org/dalibor-matura/exchange-rate)|[![codecov](https://codecov.io/gh/dalibor-matura/exchange-rate/branch/master/graph/badge.svg)](https://codecov.io/gh/dalibor-matura/exchange-rate)

## Overview

In order to provide customers a product that lets them spend cryptocurrencies to buy goods from merchants who only accept fiat currency, we need to solve two problems:
1. Determine a sequence of trades and transfers across exchanges to convert the cryptocurrency to fiat currency with a suitable exchange rate.
2. Provide the best possible exchange rate to customers.

The idea is based on the challenge I received once as a part of an interview.

## Build and Run the code

As a standard Rust project, building it and running it is what you would expect.

### Build

`cargo build`
or
`cargo build --release`

### Run

`cargo run < data/exchange-rate-path-input.txt`
or
`cargo run --release < data/exchange-rate-path-input.txt`

## Input format

Input is formed by two types of lines: "Price Update" and "Exchange Rate Request". Example of the expected input format can be found in [data/exchange-rate-input.txt](https://github.com/dalibor-matura/exchange-rate/blob/master/data/exchange-rate-input.txt).

#### Price Update

* Format: `<timestamp> <exchange> <source_currency> <destination_currency> <forward_factor> <backward_factor>`
* Example: `2019-08-01T08:42:22+00:00 BITFINEX BTC USD 1000.0 0.0009`

#### Exchange Rate Request

* Format: `EXCHANGE_RATE_REQUEST <source_exchange> <source_currency> <destination_exchange> <destination_currency>`
* Example: `EXCHANGE_RATE_REQUEST BITFINEX ETH BINANCE BTC`

## Design

The implementation consist from three main parts and a gel connecting them together.

### 1.) Graph

I decided not to use Petgraph create ([petgraph](https://crates.io/crates/petgraph)) directly, but ruther to extract and refactor `GraphMap` into my own crate called `Safe Graph` ([safe-graph](https://crates.io/crates/safe-graph)). My reasing for that is explained there.

### 2.) Floyd-Warshall algorithm

A generic solution for Floyd-Warshall algorithm supporting customization is provided by my own crate `Floyd Warshall algorithm` ([floyd-warshall-alg](https://crates.io/crates/floyd-warshall-alg)). Available customization is described there.

### 3.) IO - reading Request, processing it and writing Response

**Input:**
Reading and parsing input from stdin to `Request` instance holding instances of `PriceUpdate` and `ExchangeRateRequest` structs.

**Processing:**
Constructing a graph, running a customized version of Floyd-Warshall algorithm and forming a Response.

**Output:**
Writing the Response holding instances of `BestRatePath` struct to stdout.

## License
Licensed under the General Public License (GPL), version 3 ([LICENSE](https://github.com/dalibor-matura/exchange-rate/blob/master/LICENSE) http://www.gnu.org/licenses/gpl-3.0.en.html).
