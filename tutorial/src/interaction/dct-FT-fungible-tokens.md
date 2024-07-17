# Fungible Tokens (FT)

Fungible Tokens have variable amounts, but always have nonce 0. They may be denominated.

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let moajs = await require('@dharitrinetwork/moajs');
let { moaSys, wallets: { alice, bob, carol } } = await moajs.setupInteractive("local-testnet");

// Issue a new fungible token
let MyToken = await moaSys.sender(alice).issueFungible("MyFungibleToken", "MYTOKEN", 1_000_00, 2);

// Check the token's identifier
console.log(MyToken.getTokenIdentifier());

// Note: if you have the token identifier, you can recall the token via:
// let MyToken = await moaSys.recallToken("MYTOKEN-a4fc62");

// Check alice's token balance
// Note: if the balance comes up as 0, wait some time and try again
await moaSys.getBalance(alice, MyToken).then(moajs.print);

// Send some tokens to bob
await moaSys.sender(alice).value(MyToken(200.0)).send(bob);

// Check alice's balance (should be 800.00 MYTOKEN)
await moaSys.getBalance(alice, MyToken).then(moajs.print);

// Check bob's balance (should be 200.00 MYTOKEN)
await moaSys.getBalance(bob, MyToken).then(moajs.print);

```
