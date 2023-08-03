//
//  AddDerivedKeysData.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 05/07/2023.
//

import Foundation

extension AddDerivedKeysData {
    init(_ preview: DdPreview) {
        keySets = [AddDerivedKeyKeySetData(preview.keySet)]
        qrPayload = preview.qr.map(\.payload)
        var errors = [AddDerivedKeysError]()
        if preview.isSomeNetworkMissing {
            errors.append(.init(errorMessage: Localizable.AddDerivedKeys.Error.networkMissing.string))
        }
        if preview.isSomeAlreadyImported {
            errors.append(.init(errorMessage: Localizable.AddDerivedKeys.Error.alreadyImported.string))
        }
        self.errors = errors
    }
}

extension AddDerivedKeyKeySetData {
    init(_ keySet: DdKeySet) {
        keySetName = keySet.seedName
        derivedKeys = keySet.derivations.map(AddDerivedKeyDerivedKeyData.init)
    }
}

struct AddDerivedKeyDerivedKeyData: Equatable {
    let path: String
    let base58: String
    let identicon: SignerImage
    let network: String
}

extension AddDerivedKeyDerivedKeyData {
    init(_ detail: DdDetail) {
        path = detail.path
        base58 = detail.base58
        identicon = detail.identicon
        network = detail.networkLogo
    }
}

struct AddDerivedKeyKeySetData: Equatable {
    let keySetName: String
    let derivedKeys: [AddDerivedKeyDerivedKeyData]
}

struct AddDerivedKeysError: Equatable, Identifiable, Hashable {
    let id = UUID()
    let errorMessage: String
}

struct AddDerivedKeysData: Equatable {
    let errors: [AddDerivedKeysError]
    let keySets: [AddDerivedKeyKeySetData]
    let qrPayload: [[UInt8]]
}
