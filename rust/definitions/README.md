
# Crate `definitions`

## Overview

This lib crate is a part of [Vault](https://github.com/paritytech/parity-signer).  

It contains main definitions used both on Vault side and on Active side. "Vault side" means everything that happens in the application itself, on the air-gapped device. This includes types used to store in database and move around the network metadata, network specs, user identities non-secret information, etc. "Active side" means whatever is related to Vault management that happens **not** on the Vault itself, but rather on network-connected device.  

Generally speaking, there are two types of the databases: cold one and hot one. Vault operates with the cold one. External database on network-connected device is the hot one. However, the cold database itself needs to be pre-generated before being introduced into Vault on loading. Therefore, when Vault side is mentioned, it could be only the cold database within Vault, but the Active side could deal with both cold database preparation prior to moving it into Vault or hot database procedures.
