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
    private let callQueue: Dispatching
    private let callbackQueue: Dispatching

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        callQueue: Dispatching = DispatchQueue(label: "NavigationInitialisationService", qos: .userInitiated),
        callbackQueue: Dispatching = DispatchQueue.main,
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
    ) {
        self.databaseMediator = databaseMediator
        self.callQueue = callQueue
        self.callbackQueue = callbackQueue
        self.seedsMediator = seedsMediator
    }

    func initialiseNavigation(verifierRemoved: Bool, completion: @escaping () -> Void) {
        callQueue.async {
            do {
                try initNavigation(
                    dbname: self.databaseMediator.databaseName,
                    seedNames: self.seedsMediator.seedNames
                )
                if verifierRemoved {
                    try historyInitHistoryNoCert()
                } else {
                    try historyInitHistoryWithCert()
                }
            } catch {}
            self.callbackQueue.async {
                completion()
            }
        }
    }
}
