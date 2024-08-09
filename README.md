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
use protoflow::derive::*;
```

### Wiring up a system or subsystem

```rust
use protoflow::blocks::{Const, Drop};
use protoflow::transports::MockTransport;
use protoflow::System;

let system = System::<MockTransport>::build(|s| {
    let source = s.block(Const::<i32>::new(s.output(), 42));
    let sink = s.block(Drop::<i32>::new(s.input()));
    s.connect(&source.output, &sink.input);
});
```

### Executing a system or subsystem

```rust
use protoflow::runtimes::StdRuntime;
use protoflow::transports::MockTransport;
use protoflow::{Runtime, System};

let system = System::<MockTransport>::build(|s| {
    /* ... build the system here ... */
});

let transport = MockTransport::new();
let mut runtime = StdRuntime::new(transport).unwrap();
let running_system = runtime.execute(system).unwrap();
```

### Authoring a trivial function block

```rust,ignore
use protoflow::derive::FunctionBlock;
use protoflow::{BlockResult, FunctionBlock, InputPort, OutputPort};

/// A block that simply echoes inputs to outputs.
#[derive(FunctionBlock, Clone)]
pub struct Echo(pub InputPort<i64>, pub OutputPort<i64>);

impl FunctionBlock<i64, i64> for Echo {
    fn compute(&self, input: i64) -> BlockResult<i64> {
        Ok(input)
    }
}
```

### Authoring a simple DROP block

```rust,ignore
use protoflow::derive::Block;
use protoflow::{Block, BlockResult, BlockRuntime, InputPort, Message};

/// A block that simply discards all messages it receives.
#[derive(Block, Clone)]
pub struct Drop<T: Message>(#[input] pub InputPort<T>);

impl<T: Message> Block for Drop<T> {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(message) = self.0.recv()? {
            drop(message);
        }
        Ok(())
    }
}
```

### Authoring a simple DELAY block

```rust,ignore
use protoflow::derive::Block;
use protoflow::{Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort, Port};
use std::time::Duration;

/// A block that passes messages through while delaying them by a fixed
/// duration.
#[derive(Block, Clone)]
pub struct Delay<T: Message> {
    /// The input message stream.
    #[input]
    pub input: InputPort<T>,

    /// The output target for the stream being passed through.
    #[output]
    pub output: OutputPort<T>,

    /// A configuration parameter for how much delay to add.
    #[parameter]
    pub delay: Duration,
}

impl<T: Message + Clone + 'static> Block for Delay<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(message) = self.input.recv()? {
            runtime.sleep_for(self.delay)?;

            if self.output.is_connected() {
                self.output.send(message)?;
            }
        }
        Ok(())
    }
}
```

## 📚 Reference

### Glossary

### Blocks

- [`Buffer`](lib/protoflow/src/blocks/buffer.rs)
- [`Const`](lib/protoflow/src/blocks/const.rs)
- [`Count`](lib/protoflow/src/blocks/count.rs)
- [`Delay`](lib/protoflow/src/blocks/delay.rs)
- [`Drop`](lib/protoflow/src/blocks/drop.rs)
- [`Random`](lib/protoflow/src/blocks/random.rs)

### Features

- [`derive`](lib/protoflow/Cargo.toml)
- [`flume`](lib/protoflow/Cargo.toml)
- [`rand`](lib/protoflow/Cargo.toml)
- [`std`](lib/protoflow/Cargo.toml)
- [`sysml`](lib/protoflow/Cargo.toml)
- [`tokio`](lib/protoflow/Cargo.toml)
- [`tracing`](lib/protoflow/Cargo.toml)
- [`web`](lib/protoflow/Cargo.toml)
- [`zeromq`](lib/protoflow/Cargo.toml)

## 👨‍💻 Development

```console
$ git clone https://github.com/artob/protoflow.git
```

- - -

[![Share on Twitter](https://img.shields.io/badge/share%20on-twitter-03A9F4?logo=twitter)](https://twitter.com/share?url=https://github.com/artob/protoflow&text=Protoflow)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/artob/protoflow&title=Protoflow)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hacker%20news-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/artob/protoflow&t=Protoflow)
[![Share on Facebook](https://img.shields.io/badge/share%20on-facebook-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/artob/protoflow)
