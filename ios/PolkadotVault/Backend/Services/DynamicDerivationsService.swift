//
//  DynamicDerivationsService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 05/07/2023.
//

import Foundation

final class DynamicDerivationsService {
    private let backendService: BackendService
    private let callQueue: Dispatching
    private let callbackQueue: Dispatching

    init(
        backendService: BackendService = BackendService(),
        callQueue: Dispatching = DispatchQueue.global(qos: .userInteractive),
        callbackQueue: Dispatching = DispatchQueue.main
    ) {
        self.backendService = backendService
        self.callQueue = callQueue
        self.callbackQueue = callbackQueue
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
        callQueue.async {
            let result: Result<MSignedTransaction, TransactionError>
            do {
                let transaction: MSignedTransaction = try signDdTransaction(payload: payload, seeds: seedPhrases)
                result = .success(transaction)
            } catch let errorDisplayed as ErrorDisplayed {
                result = .failure(errorDisplayed.transactionError)
            } catch {
                result = .failure(.generic(error.backendDisplayError))
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }
}
