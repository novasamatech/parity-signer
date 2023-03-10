//
//  NavigationInitialisationService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 06/03/2023.
//

import Foundation

final class NavigationInitialisationService {
    private let seedsMediator: SeedsMediating
    private let databaseMediator: DatabaseMediating

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
    ) {
        self.databaseMediator = databaseMediator
        self.seedsMediator = seedsMediator
    }

    func initialiseNavigation(verifierRemoved: Bool) {
        do {
            try initNavigation(
                dbname: databaseMediator.databaseName,
                seedNames: seedsMediator.seedNames
            )
            if verifierRemoved {
                try historyInitHistoryNoCert()
            } else {
                try historyInitHistoryWithCert()
            }
        } catch {}
    }
}
