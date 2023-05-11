//
//  MKeyAndNetworkCard+PathAndNetwork.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 28/10/2022.
//

import Foundation

extension MKeyAndNetworkCard {
    var asPathAndNetwork: PathAndNetwork {
        PathAndNetwork(
            derivation: key.address.path,
            networkSpecsKey: network.networkSpecsKey
        )
    }

    /// To be used for Rust navigation to present Public Key Details view
    var publicKeyDetails: String {
        "\(key.addressKey)\n\(network.networkSpecsKey)"
    }
}
