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
}
