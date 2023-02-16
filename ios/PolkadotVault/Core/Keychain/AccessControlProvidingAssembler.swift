//
//  AccessControlProvidingAssembler.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 25/08/2022.
//

import Foundation

/// Assembler that prepares dependency for `AccessFlagProviding`
final class AccessControlProvidingAssembler {
    private let runtimePropertiesProvider: RuntimePropertiesProviding

    init(runtimePropertiesProvider: RuntimePropertiesProviding = RuntimePropertiesProvider()) {
        self.runtimePropertiesProvider = runtimePropertiesProvider
    }

    func assemble() -> AccessControlProviding {
        if runtimePropertiesProvider.isInDevelopmentMode {
            return SimulatorAccessControlProvider()
        } else {
            return AccessControlProvider()
        }
    }
}
