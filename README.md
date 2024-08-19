# Protoflow

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.70%2B-blue)](https://rust-lang.org)
[![Package](https://img.shields.io/crates/v/protoflow)](https://crates.io/crates/protoflow)

🚧 _We are building in public. This is presently under heavy construction._

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.70+

## ⬇️ Installation

### Installation via Cargo

```console
$ cargo install protoflow
```

## 👉 Examples

### Importing the library

```rust
use protoflow::*;
use protoflow::derive::*;
```

### Wiring up a system or subsystem

```rust
use protoflow::blocks::{Const, Drop, System, SystemBuilding};

let system = System::build(|s| {
    let source = s.block(Const::<i32>::with_params(s.output(), 42));
    let sink = s.block(Drop::<i32>::new(s.input()));
    s.connect(&source.output, &sink.input);
});
```

### Executing a system or subsystem

```rust
use protoflow::runtimes::StdRuntime;
use protoflow::transports::MpscTransport;
use protoflow::{Runtime, System};

let system = System::<MpscTransport>::build(|s| {
    /* ... build the system here ... */
});

let transport = MpscTransport::new();
let mut runtime = StdRuntime::new(transport).unwrap();
let process = runtime.execute(system).unwrap();
```

### Authoring a trivial function block

```rust
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

impl<T: Message> Block for Delay<T> {
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

| Block           | Description                                                |
| :-------------- | :--------------------------------------------------------- |
| [`Buffer`]      | Stores all messages it receives.                           |
| [`Const`]       | Sends a constant value.                                    |
| [`Count`]       | Counts the number of messages it receives, while optionally passing them through. |
| [`Decode`]      | Decodes messages from a byte stream.                       |
| [`Delay`]       | Passes messages through while delaying them by a fixed or random duration. |
| [`Drop`]        | Discards all messages it receives.                         |
| [`Encode`]      | Encodes messages to a byte stream.                         |
| [`Random`]      | Generates and sends a random value.                        |
| [`ReadDir`]     | Reads file names from a file system directory.             |
| [`ReadEnv`]     | Reads the value of an environment variable.                |
| [`ReadFile`]    | Reads bytes from the contents of a file.                   |
| [`ReadStdin`]   | Reads bytes from standard input (aka stdin).               |
| [`WriteFile`]   | Writes or appends bytes to the contents of a file.         |
| [`WriteStderr`] | Writes bytes to standard error (aka stderr).               |
| [`WriteStdout`] | Writes bytes to standard output (aka stdout).              |

### Features

- [`blocks`](lib/protoflow/Cargo.toml)
- [`crossbeam`](lib/protoflow/Cargo.toml)
- [`derive`](lib/protoflow/Cargo.toml)
- [`flume`](lib/protoflow/Cargo.toml)
- [`rand`](lib/protoflow/Cargo.toml)
- [`std`](lib/protoflow/Cargo.toml)
- [`syntax`](lib/protoflow/Cargo.toml)
- [`sysml`](lib/protoflow/Cargo.toml)
- [`tracing`](lib/protoflow/Cargo.toml)
- [`web`](lib/protoflow/Cargo.toml)
- [`zeromq`](lib/protoflow/Cargo.toml)

## 👨‍💻 Development

```console
$ git clone https://github.com/AsimovPlatform/protoflow.git
```

- - -

[![Share on Twitter](https://img.shields.io/badge/share%20on-twitter-03A9F4?logo=twitter)](https://twitter.com/share?url=https://github.com/AsimovPlatform/protoflow&text=Protoflow)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/AsimovPlatform/protoflow&title=Protoflow)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hacker%20news-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/AsimovPlatform/protoflow&t=Protoflow)
[![Share on Facebook](https://img.shields.io/badge/share%20on-facebook-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/AsimovPlatform/protoflow)

[`Buffer`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.Buffer.html
[`Const`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.Const.html
[`Count`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.Count.html
[`Decode`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.Decode.html
[`Delay`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.Delay.html
[`Drop`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.Drop.html
[`Encode`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.Encode.html
[`Random`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.Random.html
[`ReadDir`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.ReadDir.html
[`ReadEnv`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.ReadEnv.html
[`ReadFile`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.ReadFile.html
[`ReadStdin`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.ReadStdin.html
[`WriteFile`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.WriteFile.html
[`WriteStderr`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.WriteStderr.html
[`WriteStdout`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.WriteStdout.html
