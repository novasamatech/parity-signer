//
//  WarningStateMediator.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 10/03/2023.
//

import Foundation

final class WarningStateMediator: ObservableObject {
    @Published var alert: Bool = false

    private let connectivityMediator: ConnectivityMediator

    init(
        connectivityMediator: ConnectivityMediator = ServiceLocator.connectivityMediator
    ) {
        self.connectivityMediator = connectivityMediator
        setUpConnectivityMonitoring()
    }

    func updateWarnings() {
        do {
            alert = try historyGetWarnings()
        } catch {
            alert = true
        }
    }

    func resetConnectivityWarnings() {
        try? historyAcknowledgeWarnings()
        _ = try? historyGetWarnings()
        alert = false
    }
}

private extension WarningStateMediator {
    func setUpConnectivityMonitoring() {
        alert = connectivityMediator.isConnectivityOn
    }
}
