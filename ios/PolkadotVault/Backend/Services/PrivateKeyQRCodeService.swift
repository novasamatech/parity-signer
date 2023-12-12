//
//  PrivateKeyQRCodeService.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 21/09/2022.
//

import Foundation

// sourcery: AutoMockable
protocol PrivateKeyQRCodeServicing: AnyObject {
    func backupViewModel(
        _ keys: MKeysNew?,
        _ completion: @escaping (Result<BackupModalViewModel, ServiceError>) -> Void
    )
}

extension PrivateKeyQRCodeService: PrivateKeyQRCodeServicing {}

final class PrivateKeyQRCodeService {
    private let seedsMediator: SeedsMediating

    init(
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
    ) {
        self.seedsMediator = seedsMediator
    }

    func backupViewModel(
        _ keys: MKeysNew?,
        _ completion: @escaping (Result<BackupModalViewModel, ServiceError>) -> Void
    ) {
        guard
            let keys,
            let root = keys.root
        else {
            completion(.failure(.init(message: Localizable.Error.PrivateKeyQRCodeService.noKeyDetails.string)))
            return
        }
        let seedName = root.address.seedName
        let seedPhrase = seedsMediator.getSeedBackup(seedName: seedName)
        guard !seedPhrase.isEmpty
        else {
            completion(.failure(.init(message: Localizable.Error.PrivateKeyQRCodeService.backupUnavailable.string)))
            return
        }
        completion(.success(BackupModalViewModel(
            header: KeySummaryViewModel(
                keyName: seedName,
                base58: root.base58.truncateMiddle()
            ),
            derivedKeys: keys.set.map { DerivedKeyOverviewViewModel($0) },
            seedPhrase: SeedPhraseViewModel(seedPhrase: seedPhrase)
        )))
    }
}
