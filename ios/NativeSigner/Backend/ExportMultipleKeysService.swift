//
//  ExportMultipleKeysService.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 18/10/2022.
//

import Foundation

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

    func exportMultipleKeySets(
        seedNames: [String],
        _ completion: @escaping (Result<AnimatedQRCodeViewModel, ServiceError>) -> Void
    ) {
        let selectedItems: [String: ExportedSet] = Dictionary(
            uniqueKeysWithValues: seedNames
                .map { ($0, ExportedSet.all) }
        )
        export(selectedItems: selectedItems, completion)
    }

    func exportRootWithDerivedKeys(
        seedName: String,
        keys: [MKeyAndNetworkCard],
        _ completion: @escaping (Result<AnimatedQRCodeViewModel, ServiceError>) -> Void
    ) {
        let selectedItems: [String: ExportedSet] = [
            seedName: .selected(s: keys.map(\.asPathAndNetwork))
        ]
        export(selectedItems: selectedItems, completion)
    }

    private func export(
        selectedItems: [String: ExportedSet],
        _ completion: @escaping (Result<AnimatedQRCodeViewModel, ServiceError>) -> Void
    ) {
        callQueue.async {
            let result: Result<AnimatedQRCodeViewModel, ServiceError>
            do {
                let qrCodes = try exportKeyInfo(
                    selectedNames: selectedItems
                ).frames
                result = .success(AnimatedQRCodeViewModel(qrCodes: qrCodes.map(\.payload)))
            } catch {
                result = .failure(.init(message: error.localizedDescription))
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }
}
