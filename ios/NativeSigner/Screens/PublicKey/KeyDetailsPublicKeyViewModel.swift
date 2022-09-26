//
//  KeyDetailsPublicKeyViewModel.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 15/09/2022.
//

import Foundation

struct KeyDetailsPublicKeyViewModel: Equatable {
    let qrCode: QRCodeContainerViewModel
    let addressFooter: QRCodeAddressFooterViewModel?
    let rootFooter: QRCodeRootFooterViewModel?
    let isKeyExposed: Bool
    let isRootKey: Bool

    init(_ keyDetails: MKeyDetails) {
        qrCode = .init(qrCode: keyDetails.qr)
        rootFooter = keyDetails.isRootKey ? .init(
            keyName: keyDetails.address.seedName,
            base58: keyDetails.address.base58
        ) : nil
        addressFooter = keyDetails.isRootKey ? nil : .init(
            identicon: keyDetails.address.identicon,
            path: [keyDetails.address.seedName, keyDetails.address.path].joined(separator: " "),
            network: keyDetails.networkInfo.networkTitle,
            base58: keyDetails.address.base58
        )
        isKeyExposed = keyDetails.address.secretExposed
        isRootKey = keyDetails.isRootKey
    }

    init(
        qrCode: QRCodeContainerViewModel,
        addressFooter: QRCodeAddressFooterViewModel?,
        rootFooter: QRCodeRootFooterViewModel?,
        isKeyExposed: Bool,
        isRootKey: Bool
    ) {
        self.qrCode = qrCode
        self.addressFooter = addressFooter
        self.rootFooter = rootFooter
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
