# Anunaya Rollups

A modular framework for desiging custom app specififc rollup.


## Overview

Anunuya Rollups is a modular framework for building custom, application-specific rollups designed to execute complex logic with high throughput and performance. Tailored for developers who need fine-grained control over execution, Anunuya enables the design of rollups that go beyond general-purpose chains—prioritizing scalability, determinism, and modularity.

With a plug-and-play architecture, Anunuya supports:

- Custom execution environments

- Flexible state transition functions

- Optimized transaction ordering

- Integration with modular blockchain stacks (e.g., DA layers, consensus layers)

Whether you're developing a high-frequency trading engine, a decentralized game backend, or a specialized DeFi protocol, Anunuya empowers you to build exactly what your application demands—without compromise.

## Sequencer

Sequencer Overview
The Anunuya Sequencer is the core component responsible for ordering transactions and driving state transitions within the rollup. Designed for high-throughput and deterministic execution, the sequencer ensures that application-specific logic is applied consistently and efficiently.

Key responsibilities of the sequencer include:

- Transaction Ordering: Batches and orders incoming transactions to ensure fair and predictable execution.

- State Transition: Applies transactions to the rollup state using a customizable state machine.

- Block Production: Produces rollup blocks at configurable intervals or based on application-specific triggers.

- Data Commitment: Optionally commits block data or state roots to a Data Availability (DA) layer or L1 chain.

The sequencer is modular and extensible, enabling integration with different consensus mechanisms, mempool strategies, and execution backends—making it ideal for apps that require specialized behavior such as MEV resistance, privacy, or low-latency processing.


![Sequencer](assets/sequencer-dark.png#gh-dark-mode-only)
![Sequencer](assets/sequencer-light.png#gh-light-mode-only)

## Documentation

For detailed documentation, please visit our [Documentation Portal](https://docs.anunaya.com).

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for more details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.



