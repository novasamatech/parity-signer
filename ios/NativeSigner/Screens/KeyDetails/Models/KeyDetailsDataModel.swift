//
//  KeyDetailsDataModel.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 31/08/2022.
//

import Foundation

struct KeyDetailsDataModel: Equatable {
    let keySummary: KeySummaryViewModel
    let derivedKeys: [DerivedKeyRowModel]

    /// Navigation action for selecting main `Address Key`
    let addressKeyNavigation: Navigation
    /// Collection of navigation actions for tapping on `Derived Key`
    let derivedKeysNavigation: [Navigation]
    /// Navigation for `Create Derived Key`
    let createDerivedKey: Navigation = .init(action: .newKey)
    /// Name of seed to be removed with `Remove Seed` action
    let removeSeed: String

    init(
        keySummary: KeySummaryViewModel,
        derivedKeys: [DerivedKeyRowModel]
    ) {
        self.keySummary = keySummary
        self.derivedKeys = derivedKeys
        addressKeyNavigation = .init(action: .goBack)
        derivedKeysNavigation = []
        removeSeed = ""
    }

    init(_ keys: MKeys) {
        keySummary = KeySummaryViewModel(
            keyName: keys.root.address.seedName,
            base58: keys.root.base58
        )
        derivedKeys = keys.set
            .sorted(by: { $0.address.path < $1.address.path && $0.addressKey < $1.addressKey })
            .map {
                DerivedKeyRowModel(
                    viewModel: DerivedKeyRowViewModel($0),
                    actionModel: DerivedKeyActionModel(
                        tapAction: .init(action: .selectKey, details: $0.addressKey)
                    )
                )
            }

        addressKeyNavigation = .init(action: .selectKey, details: keys.root.addressKey)
        derivedKeysNavigation = keys.set
            .sorted(by: { $0.address.path < $1.address.path })
            .map { .init(action: .selectKey, details: $0.addressKey) }
        removeSeed = keys.root.address.seedName
    }
}

struct DerivedKeyExportModel: Equatable {
    let viewModel: DerivedKeyRowViewModel
    let keyData: MKeyAndNetworkCard
}
