
# Crate `definitions`

## Overview

This lib crate is a part of [Signer](https://github.com/paritytech/parity-signer).  

It contains main definitions used both on Signer side and on Active side. "Signer side" means everything that happens in the application itself, on the air-gapped device. This includes types used to store in database and move around the network metadata, network specs, user identities non-secret information, etc. "Active side" means whatever is related to Signer management that happens **not** on the Signer itself, but rather on network-connected device.  

Generally speaking, there are two types of the databases: cold one and hot one. Signer operates with the cold one. External database on network-connected device is the hot one. However, the cold database itself needs to be pre-generated before being introduced into Signer on loading. Therefore, when Signer side is mentioned, it could be only the cold database within Signer, but the Active side could deal with both cold database preparation prior to moving it into Signer or hot database procedures.
