//
//  ConnectivityMonitoringAdapter.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 02/08/2022.
//

import Foundation
import Network

/// Protocol for monitoring system connectivity
protocol ConnectivityMonitoring: AnyObject {
    /// Starts monitoring network connectivity
    /// - Parameter update: update callback informing about connectivity change
    func startMonitoring(_ update: @escaping (Bool) -> Void)
}

/// Adapter that monitors for connectivity changes
final class ConnectivityMonitoringAdapter: ObservableObject, ConnectivityMonitoring {
    private let adaptee: PathMonitorProtocol
    private let monitoringQueue: DispatchQueue
    private let notificationQueue: DispatchQueue
    private var isConnected: Bool = false

    init(
        adaptee: PathMonitorProtocol = NWPathMonitor(),
        monitoringQueue: DispatchQueue = DispatchQueue.global(qos: .background),
        notificationQueue: DispatchQueue = DispatchQueue.main
    ) {
        self.adaptee = adaptee
        self.monitoringQueue = monitoringQueue
        self.notificationQueue = notificationQueue
    }

    func startMonitoring(_ update: @escaping (Bool) -> Void) {
        adaptee.pathUpdateHandler = { [weak self] path in
            guard let self else { return }
            let isConnected = !path.availableInterfaces.isEmpty

            // Update only on connectivity changes
            guard isConnected != self.isConnected else { return }
            self.isConnected = isConnected
            notificationQueue.async {
                if isConnected {
                    try? historyDeviceWasOnline()
                }
                update(isConnected)
            }
        }
        adaptee.start(queue: monitoringQueue)
    }
}

/// Stub that gives control over connectivity changes for testing purposes
final class ConnectivityMonitoringStub: ConnectivityMonitoring {
    /// Retained update callback to be used within `triggerUpdateChange` for testing connectivity changes
    private var updateCallback: ((Bool) -> Void)?

    /// Stubbed connectivity state to be used in Development Mode
    var stubConnectivityState = false

    func startMonitoring(_ update: @escaping (Bool) -> Void) {
        updateCallback = update
        update(stubConnectivityState)
    }

    /// Utility function to trigger updated state callback if needed
    /// Should be only used for testing purposes
    func triggerUpdateChange() {
        updateCallback?(stubConnectivityState)
    }
}
