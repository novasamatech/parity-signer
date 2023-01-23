//
//  ConnectivityMediator.swift
//  NativeSigner
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
            guard let self = self else { return }
            if isConnected, self.databaseMediator.isDatabaseAvailable() {
                try? historyDeviceWasOnline()
            }
            self.isConnectivityOn = isConnected
        }
    }
}
