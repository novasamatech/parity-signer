//
//  ExportPrivateKeyService.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 12/09/2022.
//

import Foundation

final class ExportPrivateKeyService {
    private let databaseMediator: DatabaseMediating
    private let seedsMediator: SeedsMediating

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
    ) {
        self.databaseMediator = databaseMediator
        self.seedsMediator = seedsMediator
    }

    func exportPrivateKey(_ keyDetails: MKeyDetails) -> ExportPrivateKeyViewModel? {
        guard let qrCode = try? generateSecretKeyQr(
            publicKey: keyDetails.pubkey,
            expectedSeedName: keyDetails.address.seedName,
            networkSpecsKey: keyDetails.networkInfo.networkSpecsKey,
            seedPhrase: seedsMediator.getSeed(seedName: keyDetails.address.seedName),
            keyPassword: nil
        ).qr else { return nil }

        return ExportPrivateKeyViewModel(
            qrCode: qrCode,
            addressFooter: .init(
                identicon: keyDetails.address.identicon,
                rootKeyName: keyDetails.address.seedName,
                path: keyDetails.address.path,
                hasPassword: keyDetails.address.hasPwd,
                network: keyDetails.networkInfo.networkTitle,
                networkLogo: keyDetails.networkInfo.networkLogo,
                base58: keyDetails.base58
            )
        )
    }
}
