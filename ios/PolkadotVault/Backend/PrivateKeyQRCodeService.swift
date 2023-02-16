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

    func backupViewModel(_ keys: MKeysNew?) -> BackupModalViewModel? {
        guard
            let keys = keys,
            let root = keys.root
        else { return nil }
        let seedName = root.address.seedName
        let seedPhrase = seedsMediator.getSeedBackup(seedName: seedName)
        guard !seedPhrase.isEmpty else { return nil }
        return BackupModalViewModel(
            header: KeySummaryViewModel(
                keyName: seedName,
                base58: root.base58.truncateMiddle()
            ),
            derivedKeys: keys.set.map { DerivedKeyOverviewViewModel($0) },
            seedPhrase: SeedPhraseViewModel(seedPhrase: seedPhrase)
        )
    }
}
