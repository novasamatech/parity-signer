//
//  KeyDetailsPublicKeyViewModel.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 15/09/2022.
//

import Foundation

struct KeyDetailsPublicKeyViewModel: Equatable {
    let qrCode: QrData
    let footer: QRCodeAddressFooterViewModel
    let isKeyExposed: Bool
    let isRootKey: Bool

    init(_ keyDetails: MKeyDetails) {
        qrCode = keyDetails.qr
        footer = .init(
            identicon: keyDetails.address.identicon,
            rootKeyName: keyDetails.address.seedName,
            path: keyDetails.address.path,
            hasPassword: keyDetails.address.hasPwd,
            network: keyDetails.networkInfo.networkTitle,
            networkLogo: keyDetails.networkInfo.networkLogo,
            base58: keyDetails.base58
        )
        isKeyExposed = keyDetails.address.secretExposed
        isRootKey = keyDetails.isRootKey
    }

    init(
        qrCode: QrData,
        footer: QRCodeAddressFooterViewModel,
        isKeyExposed: Bool,
        isRootKey: Bool
    ) {
        self.qrCode = qrCode
        self.footer = footer
        self.isKeyExposed = isKeyExposed
        self.isRootKey = isRootKey
    }
}

struct KeyDetailsPublicKeyActionModel: Equatable {
    /// Name of seed to be removed with `Remove Seed` action
    let removeSeed: String

    init(_ keyDetails: MKeyDetails) {
        removeSeed = keyDetails.address.seedName
    }

    init(removeSeed: String) {
        self.removeSeed = removeSeed
    }
}
