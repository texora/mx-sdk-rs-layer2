# Ping-pong

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let moajs = await require('@dharitrinetwork/moajs');
let { moaSys, Moa, wallets: { alice, bob, carol, dan } } = await moajs.setupInteractive("local-testnet");

let pingPong = await moaSys.loadWrapper("contracts/examples/ping-pong-moa");

await pingPong.sender(alice).gas(150_000_000).call.deploy(Moa(0.5), 2 * 60, null, Moa(1.5));

await pingPong.gas(20_000_000).sender(alice).value(Moa(0.5)).ping("note 1");

await pingPong.sender(bob).value(Moa(0.5)).ping(null);
await pingPong.sender(carol).value(Moa(0.5)).ping(null);

// this fails because of the balance limit of 1.5 moa
await pingPong.sender(dan).value(Moa(0.5).ping(null);

await pingPong.pongAll();

```
