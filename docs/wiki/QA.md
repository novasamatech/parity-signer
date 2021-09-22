# QA Checking List

## Identity Manipulation

* Identity could be generated with 12 words recovery phrase
* Identity could be generated with 24 words recovery phrase
* Identity could be successfully recovered
* Substrate account could be generated and derivated
* An Root account could be derived by the custom (Add Network Account -> Create Custom Path -> Use empty string as path)
* An Passworded account could be derived by the custom (Add Network Account -> Create Custom Path -> Add additional password -> input any path)

## Signing
* Identity Kusama Account Extrinsic Signning with Single QR code
* Identity Polkadot Account Extrinsic Signning with Single QR code
* Identity Kusama Account Extrinsic Signning with Multiple QR code
* Identity Polkadot Account Extrinsic Signning with Multiple QR code
* After move the app into background and back to app, Signing will need pin input again.
* Identity Substrate Account with Password Signing should also work, which should always need password input
