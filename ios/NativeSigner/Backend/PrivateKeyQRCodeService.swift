//
//  PrivateKeyQRCodeService.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 21/09/2022.
//

import Foundation

final class PrivateKeyQRCodeService {
    private let databaseMediator: DatabaseMediating
    private let seedsMediator: SeedsMediating
    private let navigation: NavigationCoordinator
    private let keys: MKeys

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
        navigation: NavigationCoordinator,
        keys: MKeys
    ) {
        self.databaseMediator = databaseMediator
        self.seedsMediator = seedsMediator
        self.navigation = navigation
        self.keys = keys
    }

    func backupViewModel() -> BackupModalViewModel? {
        guard
            case let .keyDetails(keyDetails) = navigation
            .performFake(navigation: .init(action: .selectKey, details: keys.root.addressKey)).screenData,
            case .keys = navigation.performFake(navigation: .init(action: .goBack)).screenData
        else { return nil }
        let seedPhrase = seedsMediator.getSeedBackup(seedName: keyDetails.address.seedName)
        guard let exportQRCode: MKeyDetails = try? generateSecretKeyQr(
            dbname: databaseMediator.databaseName,
            publicKey: keyDetails.pubkey,
            expectedSeedName: keyDetails.address.seedName,
            networkSpecsKey: keyDetails.networkInfo.networkSpecsKey,
            seedPhrase: seedPhrase,
            keyPassword: nil
        ) else { return nil }

        return BackupModalViewModel(
            header: KeySummaryViewModel(
                keyName: keys.root.address.seedName,
                base58: keys.root.base58.truncateMiddle()
            ),
            derivedKeys: keys.set.map { DerivedKeyOverviewViewModel($0) },
            seedPhrase: SeedPhraseViewModel(seedPhrase: seedPhrase),
            qrCode: QRCodeContainerViewModel(qrCode: exportQRCode.qr)
        )
    }
}
