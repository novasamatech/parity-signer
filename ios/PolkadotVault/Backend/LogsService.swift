//
//  LogsService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 20/03/2023.
//

import Foundation

final class LogsService {
    private let databaseMediator: DatabaseMediating
    private let callQueue: Dispatching
    private let callbackQueue: Dispatching

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        callQueue: Dispatching = DispatchQueue(label: "LogsService", qos: .userInitiated),
        callbackQueue: Dispatching = DispatchQueue.main
    ) {
        self.databaseMediator = databaseMediator
        self.callQueue = callQueue
        self.callbackQueue = callbackQueue
    }

    func getLogs(_ completion: @escaping (Result<MLog, ServiceError>) -> Void) {
        callQueue.async {
            let result: Result<MLog, ServiceError>
            do {
                let logs: MLog = try PolkadotVault.getLogs()
                result = .success(logs)
            } catch {
                result = .failure(.init(message: error.localizedDescription))
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }

    func getLogDetails(
        _ logIndex: UInt32,
        _ completion: @escaping (Result<MLogDetails, ServiceError>) -> Void
    ) {
        callQueue.async {
            let result: Result<MLogDetails, ServiceError>
            do {
                let logDetails: MLogDetails = try PolkadotVault.getLogDetails(order: logIndex)
                result = .success(logDetails)
            } catch {
                result = .failure(.init(message: error.localizedDescription))
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }

    func cleaLogHistory(
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        callQueue.async {
            let result: Result<Void, ServiceError>
            do {
                try PolkadotVault.clearLogHistory()
                result = .success(())
            } catch {
                result = .failure(.init(message: error.localizedDescription))
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }

    func addCommentToLogs(
        _ userComment: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        callQueue.async {
            let result: Result<Void, ServiceError>
            do {
                try PolkadotVault.handleLogComment(userInput: userComment)
                result = .success(())
            } catch {
                result = .failure(.init(message: error.localizedDescription))
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }
}
