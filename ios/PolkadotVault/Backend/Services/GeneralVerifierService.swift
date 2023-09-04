//
//  GeneralVerifierService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 08/05/2023.
//

import Foundation

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
