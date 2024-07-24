# Protoflow

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.70%2B-blue)](https://rust-lang.org)
[![Package](https://img.shields.io/crates/v/protoflow)](https://crates.io/crates/protoflow)

🚧 _This is presently under heavy construction._

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.70+

## ⬇️ Installation

### Installation via Cargo

```console
$ cargo add protoflow
```

## 👉 Examples

### Importing the library

```rust
use protoflow::*;
```

### Authoring a simple DROP block

```rust
use protoflow::derive::Block;
use protoflow::{Block, BlockError, InputPort, Message, PortDescriptor, Scheduler};

/// A block that simply discards all messages it receives.
#[derive(Block)]
pub struct Drop<T: Message>(#[input] InputPort<T>);

impl<T: Message> Block for Drop<T> {
    fn execute(&mut self, _scheduler: &dyn Scheduler) -> Result<(), BlockError> {
        while let Some(message) = self.0.receive()? {
            drop(message);
        }
        Ok(())
    }
}
```

### Authoring a simple DELAY block

```rust
use protoflow::derive::Block;
use protoflow::{Block, BlockError, InputPort, Message, OutputPort, Port, PortDescriptor, Scheduler};
use std::time::Duration;

/// A block that passes messages through while delaying them by a fixed
/// duration.
#[derive(Block)]
pub struct Delay<T: Message> {
    /// The input message stream.
    #[input]
    input: InputPort<T>,
    /// The output target for the stream being passed through.
    #[output]
    output: OutputPort<T>,
    /// A configuration parameter for how much delay to add.
    #[parameter]
    delay: Duration,
}

impl<T: Message> Block for Delay<T> {
    fn execute(&mut self, scheduler: &dyn Scheduler) -> Result<(), BlockError> {
        while let Some(message) = self.input.receive()? {
            scheduler.sleep_for(self.delay)?;

            if self.output.is_connected() {
                self.output.send(&message)?;
            }
        }
        Ok(())
    }
}
```

### Authoring a trivial function block

```rust
use protoflow::derive::FunctionBlock;
use protoflow::{BlockError, FunctionBlock, InputPort, OutputPort};

/// A block that simply echoes inputs to outputs.
#[derive(FunctionBlock)]
pub struct Echo(InputPort<i64>, OutputPort<i64>);

impl FunctionBlock<i64, i64> for Echo {
    fn compute(&self, input: i64) -> Result<i64, BlockError> {
        Ok(input)
    }
}
```

## 📚 Reference

### Features

- [`flume`](lib/protoflow/Cargo.toml)
- [`rand`](lib/protoflow/Cargo.toml)
- [`std`](lib/protoflow/Cargo.toml)
- [`tokio`](lib/protoflow/Cargo.toml)
- [`tracing`](lib/protoflow/Cargo.toml)
- [`web`](lib/protoflow/Cargo.toml)
- [`zeromq`](lib/protoflow/Cargo.toml)

### Runtimes

- [`runtimes::StdThread`](lib/protoflow/src/runtimes/std_thread.rs)
- [`runtimes::Tokio`](lib/protoflow/src/runtimes/tokio.rs)
- [`runtimes::Web`](lib/protoflow/src/runtimes/web.rs)

### Transports

- [`transports::Flume`](lib/protoflow/src/transports/flume.rs)
- [`transports::ZeroMQ`](lib/protoflow/src/transports/zeromq.rs)

### Blocks

TODO

## 👨‍💻 Development

```console
$ git clone https://github.com/artob/protoflow.git
```

- - -

[![Share on Twitter](https://img.shields.io/badge/share%20on-twitter-03A9F4?logo=twitter)](https://twitter.com/share?url=https://github.com/artob/protoflow&text=Protoflow)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/artob/protoflow&title=Protoflow)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hacker%20news-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/artob/protoflow&t=Protoflow)
[![Share on Facebook](https://img.shields.io/badge/share%20on-facebook-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/artob/protoflow)
