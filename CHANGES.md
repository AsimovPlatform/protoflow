# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.3.1 - 2024-09-25
### Added
- Implement `Message` for `google.protobuf` well-known types

## 0.3.0 - 2024-09-09
### Added
- `MaybeNamed` and `MaybeLabeled` for blocks & ports
- `ParameterDescriptor`
### Fixed
- `PortDescriptor` field visibility

## 0.2.2 - 2024-09-07
### Added
- `PortDescriptor#direction`
- `PortDescriptor#r#type`
### Fixed
- `#[derive(FunctionBlock)]`

## 0.2.1 - 2024-08-24
### Added
- Optional Serde support for block parameters

## 0.2.0 - 2024-08-21
### Added
- `EncodeHex`: A block for hex encoding
- `Hash`: A block for BLAKE3 hashing
- Implement `BlockRuntime#wait_for()`
- Add shell examples for all blocks
### Changed
- `Count`: Ignore a disconnected port when sending the counter
- Close all ports automatically on block exit
- Send EOS prior to disconnecting the ports
- Rewrite the MPSC transport for robustness
### Fixed
- `Decode`: Fix line decoding to strip off trailing newlines

## 0.1.1 - 2024-08-20
### Added
- Implement `System#decode_with::<T>(Encoding::TextWithNewlineSuffix)`
- A new example: [`echo_lines`](lib/protoflow/examples/echo_lines)
- A new example: [`count_lines`](lib/protoflow/examples/count_lines)

## 0.1.0 - 2024-08-20
