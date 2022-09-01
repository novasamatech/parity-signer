//
//  KeyDetailsViewModel.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 31/08/2022.
//

import Foundation

struct KeyDetailsViewModel: Equatable {
    let keyName: String
    let base58: String
    let derivedKeys: [DerivedKeyRowModel]

    init(
        keyName: String,
        base58: String,
        derivedKeys: [DerivedKeyRowModel]
    ) {
        self.keyName = keyName
        self.base58 = base58
        self.derivedKeys = derivedKeys
    }

    init(_ keys: MKeys) {
        keyName = keys.root.seedName
        base58 = keys.root.base58.truncateMiddle(length: 8)
        derivedKeys = keys.set
            .sorted(by: { $0.path < $1.path })
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
