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
    private let databaseMediator: DatabaseMediating

    init(
        backendService: BackendService = BackendService(),
        databaseMediator: DatabaseMediating = DatabaseMediator()
    ) {
        self.backendService = backendService
        self.databaseMediator = databaseMediator
    }

    func checkDatabaseScheme(
        _ completion: @escaping (Result<Void, DatabaseCheckError>) -> Void
    ) {
        guard databaseMediator.isDatabaseAvailable() else { completion(.success(()))
            return
        }
        backendService.performCall {
            try checkDbVersion()
        } completion: { (result: Result<Void, ErrorDisplayed>) in
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
        }
    }
}
