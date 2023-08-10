//
//  DynamicDerivationsService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 05/07/2023.
//

import Foundation

final class DynamicDerivationsService {
    private let databaseMediator: DatabaseMediating
    private let callQueue: Dispatching
    private let callbackQueue: Dispatching

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        callQueue: Dispatching = DispatchQueue(label: "DynamicDerivationsService", qos: .userInitiated),
        callbackQueue: Dispatching = DispatchQueue.main
    ) {
        self.databaseMediator = databaseMediator
        self.callQueue = callQueue
        self.callbackQueue = callbackQueue
    }

    func getDynamicDerivationsPreview(
        for seedPhrases: [String: String],
        payload: String,
        completion: @escaping (Result<DdPreview, ServiceError>) -> Void
    ) {
        callQueue.async {
            let result: Result<DdPreview, ServiceError>
            do {
                let preview: DdPreview = try previewDynamicDerivations(seeds: seedPhrases, payload: payload)
                result = .success(preview)
            } catch {
                result = .failure(.init(message: error.backendDisplayError))
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
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
