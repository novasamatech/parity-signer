//
//  MKeyDetails+Generate.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 08/12/2023.
//

import Foundation
@testable import PolkadotVault

extension MKeyDetails {
    static func generate(
        qr: QrData = QrData.generate(),
        pubkey: String = "defaultPubkey",
        networkInfo: MscNetworkInfo = MscNetworkInfo.generate(),
        address: Address = Address.generate(),
        base58: String = "defaultBase58"
    ) -> MKeyDetails {
        MKeyDetails(
            qr: qr,
            pubkey: pubkey,
            networkInfo: networkInfo,
            address: address,
            base58: base58
        )
    }
}

extension QrData {
    static func generate(
        dataType: DataType = .regular,
        data: [UInt8] = [0, 1, 2, 3]
    ) -> QrData {
        switch dataType {
        case .regular:
            .regular(data: data)
        case .sensitive:
            .sensitive(data: data)
        }
    }
}

extension MscNetworkInfo {
    static func generate(
        networkTitle: String = "Polkadot",
        networkLogo: String = "polkadot",
        networkSpecsKey: String = "defaultSpecsKey"
    ) -> MscNetworkInfo {
        MscNetworkInfo(
            networkTitle: networkTitle,
            networkLogo: networkLogo,
            networkSpecsKey: networkSpecsKey
        )
    }
}

extension Address {
    static func generate(
        path: String = "//polkadot//0",
        hasPwd: Bool = false,
        identicon: Identicon = Identicon.generate(),
        seedName: String = "Main Key Set",
        secretExposed: Bool = false
    ) -> Address {
        Address(
            path: path,
            hasPwd: hasPwd,
            identicon: identicon,
            seedName: seedName,
            secretExposed: secretExposed
        )
    }
}

extension Identicon {
    static func generate(
        type: IdenticonType = .dots,
        identity: [UInt8] = [0, 1, 2, 3],
        identityString: String = "identity"
    ) -> Identicon {
        switch type {
        case .dots:
            .dots(identity: identity)
        case .blockies:
            .blockies(identity: identityString)
        case .jdenticon:
            .jdenticon(identity: identityString)
        }
    }

    enum IdenticonType {
        case dots
        case blockies
        case jdenticon
    }
}
