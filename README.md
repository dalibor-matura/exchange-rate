# The Exchange Rate Path Problem

*TenX Technical Exercise*

In order to provide our customers a product that lets them spend cryptocurrencies to
buy goods from merchants who only accept fiat currency, we need to solve two
problems:
1. Determine a sequence of trades and transfers across exchanges to convert
the cryptocurrency to fiat currency with a suitable exchange rate
2. Provide the best possible exchange rate to our customers

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

## Design

The implementation consist from three main parts and a gel connecting them together.

### 1.) Graph

I’ve decided to not to use petgraph https://crates.io/crates/petgraph directly, but ruther to extract and refactor “GraphMap” into my own module.

**Reasons:**
* Only the `GraphMap` was required (I renamed it to simply `Graph`).
* Petgraph has almost no tests and I’m trying to have a high/full test coverage, so I’ve added tests in.
* Petgraph has `ordermap` crate as a dependency, but it is outdated now and not stable. Its current stable version was renamed to `indexmap` and that one I’ve used.
* I’ve done a few other modifications according to best practice.
* The security implication of using a third-party crate with no stable release yet are bad, risking malicious actions by outside influence.

### 2.) Floyd-Warshall algorithm

A generic solution for Floyd-Warshall algorithm supporting substitution.

**Substitutions:**
* *operator* - being  used for weighted edges (e.g. replacing addition to concatenate paths by multiplication)
* *comparison* - being used for weighted paths to determine if a newly tested path through `k` should be added or not (e.g. replacing min operator with max)

### 3.) IO - reading Request, processing it and writing Response

**Input:**
Reading and parsing input from stdin to `Request` instance holding instances of `PriceUpdate` and `ExchangeRateRequest` structs.

**Processing:**
Constructing a graph, running a customized version of Floyd-Warshall algorithm and forming a Response.

**Output:**
Writing the Response holding instances of `BestRatePath` struct to stdout.

