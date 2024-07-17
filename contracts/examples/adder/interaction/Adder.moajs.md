# Adder

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let moajs = await require('@dharitrinetwork/moajs');
let { moaSys, wallets: { alice } } = await moajs.setupInteractive("local-testnet");

let adder = await moaSys.loadWrapper("contracts/examples/adder");

// Deploy the adder contract with an initial value of 42
await adder.sender(alice).gas(20_000_000).call.deploy(42);

// Check that the sum is 42
await adder.query.getSum().then((sum) => sum.toString());

await adder.gas(3_000_000).call.add(30);

// Check that the sum is 72
await adder.query.getSum().then((sum) => sum.toString());

```
