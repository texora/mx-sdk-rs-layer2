# Basics - explained

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let moajs = await require('@dharitrinetwork/moajs');
let { moaSys, Moa, wallets: { alice, bob, carol, dan, eve } } = await moajs.setupInteractive("local-testnet");
```

The `setupInteractive` call does several essential things:
- synchronizes the default `NetworkConfig` with the chosen provider
- loads the test wallets from the filesystem and *synchronizes their nonce*
- loads `moaSys`, which contains the DCT system smart contract and builtin functions (required for DCT issuing, transfers)
- returns `Moa` which can be used to build MOA sums (eg. `Moa(0.5)`)

## Choosing a provider

For `moajs.setupInteractive` the available providers are:
- Local Testnet proxy: `"local-testnet"`
- Dharitri Testnet proxy: `"dharitri-testnet"`
- Dharitri Devnet proxy: `"dharitri-devnet"`
- Dharitri Mainnet proxy: `"dharitri-mainnet"`

# Notes

## On working with balances

There are two ways of thinking about a balance:
- as a denominated unit (eg. 1.5 MOA)
- by its raw decimal representation (eg. "1500000000000000000")

When working with examples, it makes most sense to deal with the denominated unit, both when providing and when reading such values.
However, when MOA amounts are returned by smart contracts they are always returned as raw decimal values.

The examples below build a `Balance` of 1.5 MOA.
```javascript
Moa(1.5).toCurrencyString();
Moa("1.5").toCurrencyString();
```

On the other hand, if you need to build a balance from a raw non-denominated value, use `Moa.raw` instead. Note that the examples below are also 1.5 MOA.
```javascript
Moa.raw(1_500_000_000_000_000_000).toCurrencyString();
Moa.raw("1500000000000000000").toCurrencyString();
```

### Notes

- Javascript allows writing numerical values with the underscore separator.

- Javascript numbers are internally floating point values and, as such, have precision issues with large values (eg. `1_500_000_000_000_000_000 + 10 == 1_500_000_000_000_000_000` is `true`). This is the reason balances are stored as integer values in smart contracts (as `BigUint`) as well as in Javascript code (through `BigNumber`, which is used by `Balance` internally).

- The number of MOA decimals is 18. By using `Moa` and `Moa.raw` correctly you shouldn't have to care about this.

- When dealing with fungible or semi-fungible DCT tokens, the number of decimals varies depending on what the token's creator chose when he made it.
