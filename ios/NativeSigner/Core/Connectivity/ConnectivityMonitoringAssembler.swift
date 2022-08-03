//
//  ConnectivityMonitoringAssembler.swift
//  NativeSigner
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
        if runtimePropertiesProvider.isInDevelopmentMode {
            return ConnectivityMonitoringStub()
        } else {
            return ConnectivityMonitoringAdapter()
        }
    }
}
