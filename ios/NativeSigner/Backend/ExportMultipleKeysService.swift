//
//  ExportMultipleKeysService.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 18/10/2022.
//

import Foundation

enum ServiceError: Error {
    case unknown
}

final class ExportMultipleKeysService {
    private let databaseMediator: DatabaseMediating
    private let callQueue: Dispatching
    private let callbackQueue: Dispatching

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        callQueue: Dispatching = DispatchQueue(label: "ExportMultipleKeysService", qos: .userInitiated),
        callbackQueue: Dispatching = DispatchQueue.main
    ) {
        self.databaseMediator = databaseMediator
        self.callQueue = callQueue
        self.callbackQueue = callbackQueue
    }

    func exportMultipleKeys(
        items: [String]? = nil,
        _ completion: @escaping (Result<AnimatedQRCodeViewModel, ServiceError>) -> Void
    ) {
        callQueue.async {
            let result: Result<AnimatedQRCodeViewModel, ServiceError>
            do {
                let qrCodes = try exportKeyInfo(dbname: DatabaseMediator().databaseName, selectedNames: items).frames
                result = .success(AnimatedQRCodeViewModel(qrCodes: qrCodes))
            } catch {
                result = .failure(.unknown)
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }
}
