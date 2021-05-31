# Wallet Portability Tutorial

## Introduction

There are multiple wallets in the polkadot ecosystem. Each wallet has different strong points.

For experience users Polkadot.js + Parity Signer might be one of the best options. 

Other wallets such as AirGap might be one of the best options for inexperienced users or for friendly usage from mobile 
devices, despite not implementing all features.

Since Polkadot uses standard technologies such as BIP39 mnemonic and key derivation it might be technically possible to port
your keys from one wallet to another, enjoying both at the same time. This can also be combined with 
related polkadot features such as proxies and controllers to achieve a great level of flexibility when managing your stash.

## Portability

Introducing your secret words generated on one wallet into another may not lead to the exact same keys being generated, as they
also depend on the derivation path (and the underlying key technology).

However, it is likely that keys can be ported from one wallet to another. Contact your wallet developers and ask for details about portability.

## Verified portability

The following portability is known to work:

| From | To | Chain | Derivation Path | Comments |
|------|----|--------|-----------------|----------|
|[AirGap](https://airgap.it/)|[Parity Signer](https://www.parity.io/signer/)|Pokadot|```//44//354//0/0/0```|Go to "Add Network Account", select "Create Custom Path". |
