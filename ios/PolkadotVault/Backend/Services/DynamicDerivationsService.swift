//
//  DynamicDerivationsService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 05/07/2023.
//

import Foundation

// sourcery: AutoMockable
protocol DynamicDerivationsServicing: AnyObject {
    func getDynamicDerivationsPreview(
        for seedPhrases: [String: String],
        payload: String,
        completion: @escaping (Result<DdPreview, ServiceError>) -> Void
    )
    func signDynamicDerivationsTransaction(
        for seedPhrases: [String: String],
        payload: [String],
        completion: @escaping (Result<MSignedTransaction, TransactionError>) -> Void
    )
}

extension DynamicDerivationsService: DynamicDerivationsServicing {}

final class DynamicDerivationsService {
    private let backendService: BackendService

    init(
        backendService: BackendService = BackendService()
    ) {
        self.backendService = backendService
    }

    func getDynamicDerivationsPreview(
        for seedPhrases: [String: String],
        payload: String,
        completion: @escaping (Result<DdPreview, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try previewDynamicDerivations(seeds: seedPhrases, payload: payload)
        }, completion: completion)
    }

    func signDynamicDerivationsTransaction(
        for seedPhrases: [String: String],
        payload: [String],
        completion: @escaping (Result<MSignedTransaction, TransactionError>) -> Void
    ) {
        backendService.performCall {
            try signDdTransaction(payload: payload, seeds: seedPhrases)
        } completion: { (result: Result<MSignedTransaction, ErrorDisplayed>) in
            switch result {
            case let .success(transaction):
                completion(.success(transaction))
            case let .failure(errorDisplayed):
                completion(.failure(errorDisplayed.transactionError))
            }
        }
    }
}
