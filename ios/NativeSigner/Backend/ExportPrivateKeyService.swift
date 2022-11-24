//
//  ExportPrivateKeyService.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 12/09/2022.
//

import Foundation

final class ExportPrivateKeyService {
    private let databaseMediator: DatabaseMediating
    private let seedsMediator: SeedsMediating
    private let keyDetails: MKeyDetails

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
        keyDetails: MKeyDetails
    ) {
        self.databaseMediator = databaseMediator
        self.seedsMediator = seedsMediator
        self.keyDetails = keyDetails
    }

    func exportPrivateKey() -> ExportPrivateKeyViewModel? {
        guard let qrCode = try? generateSecretKeyQr(
            dbname: databaseMediator.databaseName,
            publicKey: keyDetails.pubkey,
            expectedSeedName: keyDetails.address.seedName,
            networkSpecsKey: keyDetails.networkInfo.networkSpecsKey,
            seedPhrase: seedsMediator.getSeed(seedName: keyDetails.address.seedName),
            keyPassword: nil
        ).qr else { return nil }

        return ExportPrivateKeyViewModel(
            qrCode: .init(qrCode: qrCode),
            addressFooter: .init(
                identicon: keyDetails.address.identicon,
                rootKeyName: keyDetails.address.seedName,
                path: keyDetails.address.path,
                network: keyDetails.networkInfo.networkTitle,
                base58: keyDetails.base58
            )
        )
    }
}
