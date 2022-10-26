//
//  DerivedKeyRowModel.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 31/08/2022.
//

import UIKit

struct DerivedKeyRowModel: Equatable {
    let viewModel: DerivedKeyRowViewModel
    let actionModel: DerivedKeyActionModel
}

/// Model of available actions for `Derived Key` cell
struct DerivedKeyActionModel: Equatable {
    /// Navigation action for tapping on `Derived Key`
    let tapAction: Navigation
}

struct DerivedKeyRowViewModel: Equatable {
    let identicon: [UInt8]
    let path: String
    let hasPassword: Bool
    let base58: String
    // for Keys Export
    let seedName: String
    let rootKeyName: String

    init(
        identicon: [UInt8],
        path: String,
        hasPassword: Bool,
        base58: String,
        seedName: String = "",
        rootKeyName: String = ""
    ) {
        self.identicon = identicon
        self.path = path
        self.hasPassword = hasPassword
        self.base58 = base58
        self.seedName = seedName
        self.rootKeyName = rootKeyName
    }
}

extension DerivedKeyRowViewModel {
    init(_ key: MKeysCard) {
        path = key.address.path
        identicon = key.address.identicon
        hasPassword = key.address.hasPwd
        base58 = key.base58
        seedName = key.addressKey
        rootKeyName = key.address.seedName
    }
}
