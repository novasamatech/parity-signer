//
//  GetAllNetworksService.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 10/01/2023.
//

import Foundation

final class GetAllNetworksService {
    private let databaseMediator: DatabaseMediating
    private let callQueue: Dispatching
    private let callbackQueue: Dispatching

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        callQueue: Dispatching = DispatchQueue(label: "GetAllNetworksService", qos: .userInitiated),
        callbackQueue: Dispatching = DispatchQueue.main
    ) {
        self.databaseMediator = databaseMediator
        self.callQueue = callQueue
        self.callbackQueue = callbackQueue
    }

    func getNetworks(
        _ completion: @escaping (Result<[MmNetwork], ServiceError>) -> Void
    ) {
        callQueue.async {
            let result: Result<[MmNetwork], ServiceError>
            do {
                let networks: [MmNetwork] = try getAllNetworks(
                    dbname: self.databaseMediator.databaseName
                )
                result = .success(networks)
            } catch {
                result = .failure(.unknown)
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }
}
