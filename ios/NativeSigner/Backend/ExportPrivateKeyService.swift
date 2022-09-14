//
//  ExportPrivateKeyService.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 12/09/2022.
//

import Foundation

final class ExportPrivateKeyService {
    private let databaseMediator: DatabaseMediating
    private weak var seedsMediator: SeedsMediating!

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        seedsMediator: SeedsMediating? = nil
    ) {
        self.databaseMediator = databaseMediator
        self.seedsMediator = seedsMediator
    }

    func exportPrivateKey(from keyDetails: MKeyDetails) -> ExportPrivateKeyViewModel! {
        guard let qrCode = try? generateSecretKeyQr(
            dbname: databaseMediator.databaseName,
            publicKey: keyDetails.pubkey,
            expectedSeedName: keyDetails.address.seedName,
            networkSpecsKey: keyDetails.networkInfo.networkSpecsKey,
            seedPhrase: seedsMediator.getSeed(seedName: keyDetails.address.seedName),
            keyPassword: nil
        ).qr else { return nil }

        return ExportPrivateKeyViewModel(
            identicon: keyDetails.address.identicon,
            qrCode: qrCode,
            path: [keyDetails.address.seedName, keyDetails.address.path].joined(separator: " "),
            network: keyDetails.networkInfo.networkTitle,
            base58: keyDetails.address.base58
        )
    }
}
