//
//  ExportKeySetService.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 18/10/2022.
//

import Foundation

final class ExportKeySetService {
    private let databaseMediator: DatabaseMediating
    private let callQueue: Dispatching
    private let callbackQueue: Dispatching

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        callQueue: Dispatching = DispatchQueue(label: "ExportKeySetService", qos: .userInitiated),
        callbackQueue: Dispatching = DispatchQueue.main
    ) {
        self.databaseMediator = databaseMediator
        self.callQueue = callQueue
        self.callbackQueue = callbackQueue
    }

    func exportRootWithDerivedKeys(
        seedName: String,
        keys: [MKeyAndNetworkCard],
        _ completion: @escaping (Result<AnimatedQRCodeViewModel, ServiceError>) -> Void
    ) {
        let selectedItems: ExportedSet = .selected(s: keys.map(\.asPathAndNetwork))
        export(seedName: seedName, selectedItems: selectedItems, completion)
    }

    private func export(
        seedName: String,
        selectedItems: ExportedSet,
        _ completion: @escaping (Result<AnimatedQRCodeViewModel, ServiceError>) -> Void
    ) {
        callQueue.async {
            let result: Result<AnimatedQRCodeViewModel, ServiceError>
            do {
                let qrCodes = try exportKeyInfo(seedName: seedName, exportedSet: selectedItems).frames
                result = .success(AnimatedQRCodeViewModel(qrCodes: qrCodes.map(\.payload)))
            } catch {
                result = .failure(.init(message: error.backendDisplayError))
            }
            self.callbackQueue.async {
                completion(result)
            }
        }
    }
}
