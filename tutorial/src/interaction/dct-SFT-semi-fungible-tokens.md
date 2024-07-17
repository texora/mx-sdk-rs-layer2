# Semi-Fungible Tokens (SFTs).

Semi-Fungible Tokens have variable amounts, and variable nonce. They are not denominated (the amount has 0 decimals).

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let moajs = await require('@dharitrinetwork/moajs');
let { moaSys, Moa, wallets: { alice, bob, carol } } = await moajs.setupInteractive("local-testnet");

// Issue a new semi-fungible token
let MyToken = await moaSys.sender(alice).issueSemiFungible("MySemiFungibleToken", "MYTOKEN");

// Check the token's identifier
console.log(MyToken.getTokenIdentifier());

await moaSys.dctSystemContract.sender(alice).call.setSpecialRole(MyToken, alice, "DCTRoleNFTCreate", "DCTRoleNFTAddQuantity");

// Create a new nonce
let MyFirstSemi = await moaSys.sender(alice).dctNftCreate(MyToken, 1_000, "MyFirstSemi", 0, "", "", "https://example.com");

// Check alice's token balance
// Note: if the balance comes up as 0, wait some time and try again
await moaSys.getBalance(alice, MyFirstSemi).then(moajs.print);

// Send some tokens to bob and carol
await moaSys.sender(alice).value(MyFirstSemi(200)).send(bob);

await moaSys.getBalance(alice, MyFirstSemi).then(moajs.print);
await moaSys.getBalance(bob, MyFirstSemi).then(moajs.print);

```
