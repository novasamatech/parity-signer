//
//  DatabaseVersionMediator.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 04/10/2023.
//

import Foundation
import SwiftUI

enum DatabaseCheckError: Error {
    case invalidVersion
    case error(ServiceError)
}

final class DatabaseVersionMediator {
    private let backendService: BackendService

    init(
        backendService: BackendService = BackendService()
    ) {
        self.backendService = backendService
    }

    func checkDatabaseScheme(
        _ completion: @escaping (Result<Void, DatabaseCheckError>) -> Void
    ) {
        backendService.performCall({
            try checkDbVersion()
        }, completion: { (result: Result<Void, ErrorDisplayed>) in
            switch result {
            case .success:
                completion(.success(()))
            case let .failure(error):
                switch error {
                case .DbSchemaMismatch:
                    completion(.failure(.invalidVersion))
                default:
                    completion(.failure(.error(.init(message: error.backendDisplayError))))
                }
            }
        })
    }
}
