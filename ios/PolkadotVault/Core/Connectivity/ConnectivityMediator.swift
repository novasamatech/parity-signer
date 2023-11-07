//
//  ConnectivityMediator.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 03/10/2022.
//

import Foundation

final class ConnectivityMediator: ObservableObject {
    private let connectivityMonitor: ConnectivityMonitoring

    @Published private(set) var isConnectivityOn: Bool = false

    init(
        connectivityMonitor: ConnectivityMonitoring = ConnectivityMonitoringAssembler().assemble()
    ) {
        self.connectivityMonitor = connectivityMonitor
        setUpConnectivityMonitoring()
    }
}

private extension ConnectivityMediator {
    func setUpConnectivityMonitoring() {
        connectivityMonitor.startMonitoring { [weak self] isConnected in
            guard let self else { return }
            isConnectivityOn = isConnected
        }
    }
}
