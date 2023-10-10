//
//  AirgapMediatorAssembler.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 17/02/2023.
//

import Foundation

/// Assembler that prepares dependency for `AirgapMediating`
final class AirgapMediatorAssembler {
    private let runtimePropertiesProvider: RuntimePropertiesProviding

    init(runtimePropertiesProvider: RuntimePropertiesProviding = RuntimePropertiesProvider()) {
        self.runtimePropertiesProvider = runtimePropertiesProvider
    }

    func assemble() -> AirgapMediating {
        if runtimePropertiesProvider.isInDevelopmentMode {
            AirgapMediatingStub()
        } else {
            AirgapMediator()
        }
    }
}
