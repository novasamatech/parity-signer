//
//  LogsService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 20/03/2023.
//

import Foundation

// sourcery: AutoMockable
protocol LogsServicing: AnyObject {
    func getLogs(_ completion: @escaping (Result<MLog, ServiceError>) -> Void)
    func getLogDetails(
        _ logIndex: UInt32,
        _ completion: @escaping (Result<MLogDetails, ServiceError>) -> Void
    )
    func cleaLogHistory(
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    )
    func addCommentToLogs(
        _ userComment: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    )
}

extension LogsService: LogsServicing {}

final class LogsService {
    private let backendService: BackendService

    init(
        backendService: BackendService = BackendService()
    ) {
        self.backendService = backendService
    }

    func getLogs(_ completion: @escaping (Result<MLog, ServiceError>) -> Void) {
        backendService.performCall({
            try PolkadotVault.getLogs()
        }, completion: completion)
    }

    func getLogDetails(
        _ logIndex: UInt32,
        _ completion: @escaping (Result<MLogDetails, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try PolkadotVault.getLogDetails(order: logIndex)
        }, completion: completion)
    }

    func cleaLogHistory(
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try PolkadotVault.clearLogHistory()
        }, completion: completion)
    }

    func addCommentToLogs(
        _ userComment: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        backendService.performCall({
            try handleLogComment(userInput: userComment)
        }, completion: completion)
    }
}
