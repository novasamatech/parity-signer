//
//  ConnectivityMediator.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 03/10/2022.
//

import Foundation

final class ConnectivityMediator: ObservableObject {
    private let connectivityMonitor: ConnectivityMonitoring
    private let databaseMediator: DatabaseMediating

    @Published private(set) var isConnectivityOn: Bool = false

    init(
        connectivityMonitor: ConnectivityMonitoring = ConnectivityMonitoringAssembler().assemble(),
        databaseMediator: DatabaseMediating = DatabaseMediator()
    ) {
        self.connectivityMonitor = connectivityMonitor
        self.databaseMediator = databaseMediator
        setUpConnectivityMonitoring()
    }
}

private extension ConnectivityMediator {
    func setUpConnectivityMonitoring() {
        connectivityMonitor.startMonitoring { [weak self] isConnected in
            guard let self else { return }
            if isConnected, databaseMediator.isDatabaseAvailable() {
                try? historyDeviceWasOnline()
            }
            isConnectivityOn = isConnected
        }
    }
}
