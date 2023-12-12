//
//  GeneralVerifierService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 08/05/2023.
//

import Foundation

// sourcery: AutoMockable
protocol GeneralVerifierServicing: AnyObject {
    func getGeneralVerifier(
        _ completion: @escaping (Result<MVerifierDetails, ServiceError>) -> Void
    )
}

extension GeneralVerifierService: GeneralVerifierServicing {}

final class GeneralVerifierService {
    private let backendService: BackendService

    init(
        backendService: BackendService = BackendService()
    ) {
        self.backendService = backendService
    }

    func getGeneralVerifier(
        _ completion: @escaping (Result<MVerifierDetails, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try getVerifierDetails()
        }, completion: completion)
    }
}
