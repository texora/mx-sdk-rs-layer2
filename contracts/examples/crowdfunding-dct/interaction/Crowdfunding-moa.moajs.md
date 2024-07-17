# Crowdfunding DCT - Using MOA

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let moajs = await require('@dharitrinetwork/moajs');
let { moaSys, Moa, wallets: { alice, bob, carol }} = await moajs.setupInteractive("local-testnet");

let crowdfunding = await moaSys.loadWrapper("contracts/examples/crowdfunding-dct");

// Set the deadline to 1 minute from now (adjust this if you want more time before claiming the rewards)
let someTimeFromNow = await moaSys.currentNonce() + moajs.minutesToNonce(1);

// Deploy the crowdfunding contract with a target of 2 MOA
await crowdfunding.sender(alice).gas(50_000_000).call.deploy(Moa(2), someTimeFromNow, Moa);

// Bob and carol contribute 1.5 MOA each
await crowdfunding.sender(bob).gas(10_000_000).value(Moa(1.5)).call.fund();
await crowdfunding.sender(carol).value(Moa(1.5)).call.fund();

// Get the current funds. Note the usage of Moa.raw (since the balance comes as an integer from the smart contract)
let currentFunds = Moa.raw(await crowdfunding.query.currentFunds());

// Should print 3 MOA (since bob and carol added 1.5 MOA each)
moajs.print(currentFunds);

// Confirming the target is 2 MOA
moajs.print(Moa.raw(await crowdfunding.query.get_target()));

// Check that alice is the owner
alice.address.equals(await crowdfunding.query.get_owner());

// Store alice's current balance (we'll use this to check the balance difference later on)
let aliceBalanceBefore = await moaSys.getBalance(alice, Moa);
moajs.print(aliceBalanceBefore);

// Wait a minute first, otherwise you'll get the "cannot claim before deadline" error
// If the claim doesn't return an error - there are two possibilities:
// - the funding failed, and 1.5 MOA are sent back to both bob and carol
// - it was succesful and alice receives 3 MOA
// Because the target sum specified on deployment was 2 MOA, and we have 3 MOA, the funding should be succesful
await crowdfunding.sender(alice).call.claim();

// Let's check if alice received the funds
let aliceBalanceAfter = await moaSys.getBalance(alice, Moa);
moajs.print(aliceBalanceAfter);

// If the previous claim was successful, this prints 2.99 MOA (because of the gas costs)
moajs.print(aliceBalanceAfter.minus(aliceBalanceBefore));
```
