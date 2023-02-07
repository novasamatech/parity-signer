//
//  DerivedKeyRowModel.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 31/08/2022.
//

import UIKit

struct DerivedKeyRowModel: Equatable {
    let keyData: MKeyAndNetworkCard
    let viewModel: DerivedKeyRowViewModel
    let actionModel: DerivedKeyActionModel
}

/// Model of available actions for `Derived Key` cell
struct DerivedKeyActionModel: Equatable {
    /// Navigation action for tapping on `Derived Key`
    let tapAction: Navigation
}

struct DerivedKeyRowViewModel: Equatable {
    let addressKey: String
    let identicon: [UInt8]
    let network: String
    let path: String
    let hasPassword: Bool
    let base58: String
    // for Keys Export
    let rootKeyName: String

    init(_ key: MKeyAndNetworkCard) {
        addressKey = key.key.addressKey
        path = key.key.address.path
        identicon = key.key.address.identicon.svgPayload
        network = key.network.networkLogo
        hasPassword = key.key.address.hasPwd
        base58 = key.key.base58
        rootKeyName = key.key.address.seedName
    }
}

extension DerivedKeyRowViewModel {
    init(
        addressKey: String = "",
        identicon: [UInt8],
        network: String,
        path: String,
        hasPassword: Bool,
        base58: String,
        rootKeyName: String = ""
    ) {
        self.addressKey = addressKey
        self.identicon = identicon
        self.network = network
        self.path = path
        self.hasPassword = hasPassword
        self.base58 = base58
        self.rootKeyName = rootKeyName
    }
}
