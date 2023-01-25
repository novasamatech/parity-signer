//
//  KeyDetailsService.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 28/10/2022.
//

import Foundation

final class KeyDetailsService {
    private let databaseMediator: DatabaseMediating
    private let callQueue: Dispatching
    private let callbackQueue: Dispatching

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        callQueue: Dispatching = DispatchQueue(label: "KeyDetailsService", qos: .userInitiated),
        callbackQueue: Dispatching = DispatchQueue.main
    ) {
        self.databaseMediator = databaseMediator
        self.callQueue = callQueue
        self.callbackQueue = callbackQueue
    }

    func getKeys(
        for seedName: String,
        _ completion: @escaping (Result<MKeysNew, ServiceError>) -> Void
    ) {
        callQueue.async {
            let result: Result<MKeysNew, ServiceError>
            do {
                let keys: MKeysNew = try keysBySeedName(seedName: seedName)
                result = .success(keys)
            } catch {
                result = .failure(.init(message: error.localizedDescription))
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }
}
