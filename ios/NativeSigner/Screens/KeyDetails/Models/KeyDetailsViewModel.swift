//
//  KeyDetailsViewModel.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 31/08/2022.
//

import Foundation

struct KeyDetailsViewModel: Equatable {
    let keySummary: KeySummaryViewModel
    let derivedKeys: [DerivedKeyRowModel]

    init(
        keySummary: KeySummaryViewModel,
        derivedKeys: [DerivedKeyRowModel]
    ) {
        self.keySummary = keySummary
        self.derivedKeys = derivedKeys
    }

    init(_ keys: MKeys) {
        keySummary = KeySummaryViewModel(
            keyName: keys.root.address.seedName,
            base58: keys.root.base58.truncateMiddle()
        )
        derivedKeys = keys.set
            .sorted(by: { $0.address.path < $1.address.path })
            .map {
                DerivedKeyRowModel(
                    viewModel: DerivedKeyRowViewModel($0),
                    actionModel: DerivedKeyActionModel(
                        tapAction: .init(action: .selectKey, details: $0.addressKey)
                    )
                )
            }
    }
}
