//
//  RecoverKeySetService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 18/04/2023.
//

import Foundation

final class RecoverKeySetService {
    private let backendService: BackendService

    init(
        backendService: BackendService = BackendService()
    ) {
        self.backendService = backendService
    }

    func updateGuessWords(
        userInput: String,
        completion: @escaping (Result<[String], ServiceError>) -> Void
    ) {
        backendService.performCall({
            seedPhraseGuessWords(userInput: userInput)
        }, completion: completion)
    }

    func validate(
        seedPhrase: String,
        completion: @escaping (Result<Bool, ServiceError>) -> Void
    ) {
        backendService.performCall({
            validateSeedPhrase(seedPhrase: seedPhrase)
        }, completion: completion)
    }
}
