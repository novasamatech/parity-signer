//
//  MNetworkDetails+Generate.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 08/01/2024.
//

import Foundation
@testable import PolkadotVault

extension MNetworkDetails {
    static func generate(
        base58prefix: UInt16 = 0,
        color: String = "defaultColor",
        decimals: UInt8 = 0,
        encryption: Encryption = .sr25519,
        genesisHash: H256 = [],
        logo: String = "defaultLogo",
        name: String = "Default Name",
        order: String = "defaultOrder",
        pathId: String = "defaultPathId",
        secondaryColor: String = "defaultSecondaryColor",
        title: String = "Default Title",
        unit: String = "defaultUnit",
        currentVerifier: MVerifier = .generate(),
        meta: [MMetadataRecord] = [.generate()]
    ) -> MNetworkDetails {
        MNetworkDetails(
            base58prefix: base58prefix,
            color: color,
            decimals: decimals,
            encryption: encryption,
            genesisHash: genesisHash,
            logo: logo,
            name: name,
            order: order,
            pathId: pathId,
            secondaryColor: secondaryColor,
            title: title,
            unit: unit,
            currentVerifier: currentVerifier,
            meta: meta
        )
    }
}

extension MVerifier {
    static func generate(
        ttype: String = "defaultType",
        details: MVerifierDetails = .generate()
    ) -> MVerifier {
        MVerifier(ttype: ttype, details: details)
    }
}

extension MVerifierDetails {
    static func generate(
        publicKey: String = "defaultPublicKey",
        identicon: Identicon = .generate(),
        encryption: String = "defaultEncryption"
    ) -> MVerifierDetails {
        MVerifierDetails(publicKey: publicKey, identicon: identicon, encryption: encryption)
    }
}

extension MMetadataRecord {
    static func generate(
        specname: String = "defaultSpecName",
        specsVersion: String = "defaultSpecsVersion",
        metaHash: String = "defaultMetaHash",
        metaIdPic: Identicon = .generate()
    ) -> MMetadataRecord {
        MMetadataRecord(
            specname: specname,
            specsVersion: specsVersion,
            metaHash: metaHash,
            metaIdPic: metaIdPic
        )
    }
}
