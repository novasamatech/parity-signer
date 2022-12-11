//
//  KeyDetailsPublicKeyViewModel.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 15/09/2022.
//

import Foundation

struct KeyDetailsPublicKeyViewModel: Equatable {
    enum Footer: Equatable {
        case root(QRCodeRootFooterViewModel)
        case address(QRCodeAddressFooterViewModel)
    }

    let qrCode: QrData
    let footer: Footer
    let isKeyExposed: Bool
    let isRootKey: Bool

    init(_ keyDetails: MKeyDetails) {
        qrCode = keyDetails.qr
        if keyDetails.isRootKey {
            footer = .root(
                .init(
                    keyName: keyDetails.address.seedName,
                    base58: keyDetails.base58
                )
            )
        } else {
            footer = .address(
                .init(
                    identicon: keyDetails.address.identicon.svgPayload,
                    rootKeyName: keyDetails.address.seedName,
                    path: keyDetails.address.path,
                    network: keyDetails.networkInfo.networkTitle,
                    base58: keyDetails.base58
                )
            )
        }
        isKeyExposed = keyDetails.address.secretExposed
        isRootKey = keyDetails.isRootKey
    }

    init(
        qrCode: QrData,
        footer: Footer,
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
