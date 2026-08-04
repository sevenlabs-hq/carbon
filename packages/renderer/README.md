# @sevenlabs-hq/carbon-codama-renderer

[![npm version](https://img.shields.io/npm/v/@sevenlabs-hq/carbon-codama-renderer.svg)](https://www.npmjs.com/package/@sevenlabs-hq/carbon-codama-renderer)
[![npm downloads](https://img.shields.io/npm/dm/@sevenlabs-hq/carbon-codama-renderer.svg)](https://www.npmjs.com/package/@sevenlabs-hq/carbon-codama-renderer)

Carbon codama renderer for generating Carbon-compatible Rust decoder code from Codama IDL files.

## Installation

```sh
npm install @sevenlabs-hq/carbon-codama-renderer
```

## Usage

This package is used internally by [`@sevenlabs-hq/carbon-cli`](https://www.npmjs.com/package/@sevenlabs-hq/carbon-cli) when processing Codama IDL files.

Codama `eventNode` values are rendered automatically. The renderer derives the
event-CPI discriminator and each event discriminator from the IDL, generates the
typed event data, and adds the `CpiEvent` instruction variant. Events encoded as
a `hiddenPrefixTypeNode` carry no event-CPI envelope in the IDL; they default to
the Anchor event-CPI discriminator `[228, 69, 165, 46, 81, 203, 154, 29]`, and
the `eventCpiDiscriminator` renderer option overrides that default for programs
with a custom envelope. The existing `anchorEvents` renderer option remains
available for Anchor IDLs that represent events as defined types.

## Links

- [NPM Package](https://www.npmjs.com/package/@sevenlabs-hq/carbon-codama-renderer)
- [Carbon CLI](https://www.npmjs.com/package/@sevenlabs-hq/carbon-cli)
- [Carbon Framework](https://github.com/sevenlabs-hq/carbon)

## License

MIT
