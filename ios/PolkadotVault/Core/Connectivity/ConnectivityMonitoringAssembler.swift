//
//  ConnectivityMonitoringAssembler.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 02/08/2022.
//

import Foundation

/// Assembler that prepares dependency for `ConnectivityMonitoring`
final class ConnectivityMonitoringAssembler {
    private let runtimePropertiesProvider: RuntimePropertiesProviding

    init(runtimePropertiesProvider: RuntimePropertiesProviding = RuntimePropertiesProvider()) {
        self.runtimePropertiesProvider = runtimePropertiesProvider
    }

    func assemble() -> ConnectivityMonitoring {
        switch runtimePropertiesProvider.runtimeMode {
        case .production,
             .qa:
            ConnectivityMonitoringAdapter()
        case .debug:
            ConnectivityMonitoringStub()
        }
    }
}
