# Preparations

# Setting up the local testnet

The following examples rely on having a [local testnet](https://docs.dharitri.com/developers/setup-local-testnet/) up and running.

# Installing @dharitrinetwork/moajs globally

```bash
cd ./code/dharitri-sdk-moajs
npm run compile && npm test && npm install -g
```

# How to start a node terminal

By exporting `NODE_PATH`, the node terminal should have access to `moajs`.
Open a terminal and enter the following:

```bash
cd ./code/dharitri-wasm-rs
export NODE_PATH=$HOME/.nvm/versions/node/$(node --version)/lib/node_modules
node --experimental-repl-await
```

# Basic DCT usage

- [Fungible Tokens (FTs)](dct-FT-fungible-tokens.md)
- [Semi-Fungible Tokens (SFTs)](dct-SFT-semi-fungible-tokens.md)
- [Non-Fungible Tokens (NFTs)](dct-NFT-non-fungible-tokens.md)

# Smart contract examples

- Adder [interaction](../../../contracts/examples/adder/interaction/Adder.moajs.md)
- Crowdfunding DCT [MOA interaction](../../../contracts/examples/crowdfunding-dct/interaction/Crowdfunding-moa.moajs.md), [DCT interaction](../../../contracts/examples/crowdfunding-dct/interaction/Crowdfunding-dct.moajs.md)
- Multisig [MOA adder interaction](../../../contracts/examples/multisig/interaction/Multisig-adder-moa.moajs.md)
- Ping-pong [MOA interaction](../../../contracts/examples/ping-pong-moa/interaction/Ping-pong-moa.moajs.md)
