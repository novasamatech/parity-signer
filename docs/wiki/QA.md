# QA Checking List

## Identity Manipulation

* Identity could be generated with 12 words recovery phrase
* Identity could be generated with 24 words recovery phrase
* Identity could be successfully recovered
* Substrate account could be generated and derivated
* Ethereum account could be generated within Identity
* An Root account could be derived by the custom (Add Network Account -> Create Custom Path -> Use empty string as path)
* An Passworded account could be derived by the custom (Add Network Account -> Create Custom Path -> Add additional password -> input any path)

## Signing
* Legacy Kusama Account Extrinsic Signning with Single QR code
* Legacy Kusama Account Extrinsic Signning with Multiple QR code
* Legacy Ethereum Account Tx Signinng with MyCrypto
* Legacy Ethereum Account Message Signinng with MyCrypto
* Identity Kusama Account Extrinsic Signning with Single QR code
* Identity Polkadot Account Extrinsic Signning with Single QR code
* Identity Kusama Account Extrinsic Signning with Multiple QR code
* Identity Polkadot Account Extrinsic Signning with Multiple QR code
* After move the app into background and back to app, Signing will need pin input again.
* Identity Substrate Account with Password Signing should also works, which should always need password input
