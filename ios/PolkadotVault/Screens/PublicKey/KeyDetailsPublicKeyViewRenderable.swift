//
//  KeyDetailsPublicKeyViewRenderable.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 26/04/2023.
//

import Foundation

struct KeyDetailsPublicKeyViewRenderable: Equatable {
    let qrCodes: [[UInt8]]
    let footer: QRCodeAddressFooterViewModel
    let isKeyExposed: Bool
    let isRootKey: Bool
    let networkTitle: String
    let networkLogo: String
    let keySetName: String
    let path: String
    let hasPassword: Bool

    init(_ keyDetails: MKeyDetails) {
        qrCodes = [keyDetails.qr.payload]
        footer = .init(
            identicon: keyDetails.address.identicon,

            networkLogo: keyDetails.networkInfo.networkLogo,
            base58: keyDetails.base58
        )
        isKeyExposed = keyDetails.address.secretExposed
        isRootKey = keyDetails.isRootKey
        networkTitle = keyDetails.networkInfo.networkTitle
        networkLogo = keyDetails.networkInfo.networkLogo
        keySetName = keyDetails.address.seedName
        path = keyDetails.address.path
        hasPassword = keyDetails.address.hasPwd
    }

    init(
        qrCodes: [[UInt8]],
        footer: QRCodeAddressFooterViewModel,
        isKeyExposed: Bool,
        isRootKey: Bool,
        networkTitle: String,
        networkLogo: String,
        keySetName: String,
        path: String,
        hasPassword: Bool
    ) {
        self.qrCodes = qrCodes
        self.footer = footer
        self.isKeyExposed = isKeyExposed
        self.isRootKey = isRootKey
        self.networkTitle = networkTitle
        self.networkLogo = networkLogo
        self.keySetName = keySetName
        self.path = path
        self.hasPassword = hasPassword
    }
}
