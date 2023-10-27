//
//  PrivateKeyQRCodeService.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 21/09/2022.
//

import Foundation

final class PrivateKeyQRCodeService {
    private let seedsMediator: SeedsMediating

    init(
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
    ) {
        self.seedsMediator = seedsMediator
    }

    func backupViewModel(
        _ keys: MKeysNew?
    ) -> Result<BackupModalViewModel, ServiceError> {
        guard
            let keys,
            let root = keys.root
        else { return .failure(.init(message: Localizable.Error.PrivateKeyQRCodeService.noKeyDetails.string)) }
        let seedName = root.address.seedName
        let seedPhrase = seedsMediator.getSeedBackup(seedName: seedName)
        guard !seedPhrase.isEmpty
        else { return .failure(.init(message: Localizable.Error.PrivateKeyQRCodeService.backupUnavailable.string)) }
        return .success(BackupModalViewModel(
            header: KeySummaryViewModel(
                keyName: seedName,
                base58: root.base58.truncateMiddle()
            ),
            derivedKeys: keys.set.map { DerivedKeyOverviewViewModel($0) },
            seedPhrase: SeedPhraseViewModel(seedPhrase: seedPhrase)
        ))
    }
}
