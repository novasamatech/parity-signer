//
//  MSufficientCryptoReady+Generate.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 10/01/2024.
//

import Foundation
@testable import PolkadotVault

extension MSufficientCryptoReady {
    static func generate(
        authorInfo: MAddressCard = MAddressCard.generate(),
        sufficient: [UInt8] = [1, 2, 3],
        content: MscContent = MscContent.addSpecs(f: .generate()),
        networkLogo: String? = "defaultNetworkLogo"
    ) -> MSufficientCryptoReady {
        MSufficientCryptoReady(
            authorInfo: authorInfo,
            sufficient: sufficient,
            content: content,
            networkLogo: networkLogo
        )
    }
}

extension MSignSufficientCrypto {
    static func generate(
        identities: [MRawKey] = [MRawKey.generate()]
    ) -> MSignSufficientCrypto {
        MSignSufficientCrypto(identities: identities)
    }
}

extension MRawKey {
    static func generate(
        address: Address = Address.generate(),
        addressKey: String = "defaultAddressKey",
        publicKey: String = "defaultPublicKey",
        networkLogo: String = "defaultNetworkLogo"
    ) -> MRawKey {
        MRawKey(
            address: address,
            addressKey: addressKey,
            publicKey: publicKey,
            networkLogo: networkLogo
        )
    }
}
