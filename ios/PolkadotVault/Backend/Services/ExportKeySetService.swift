//
//  ExportKeySetService.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 18/10/2022.
//

import Foundation

// sourcery: AutoMockable
protocol ExportKeySetServicing: AnyObject {
    func exportRootWithDerivedKeys(
        seedName: String,
        keys: [MKeyAndNetworkCard],
        _ completion: @escaping (Result<AnimatedQRCodeViewModel, ServiceError>) -> Void
    )
    func exportRoot(
        seedName: String,
        _ completion: @escaping (Result<AnimatedQRCodeViewModel, ServiceError>) -> Void
    )
}

extension ExportKeySetService: ExportKeySetServicing {}

final class ExportKeySetService {
    private let backendService: BackendService

    init(
        backendService: BackendService = BackendService()
    ) {
        self.backendService = backendService
    }

    func exportRootWithDerivedKeys(
        seedName: String,
        keys: [MKeyAndNetworkCard],
        _ completion: @escaping (Result<AnimatedQRCodeViewModel, ServiceError>) -> Void
    ) {
        let selectedItems: ExportedSet = .selected(s: keys.map(\.asPathAndNetwork))
        backendService.performCall({
            let qrCodes = try exportKeyInfo(seedName: seedName, exportedSet: selectedItems).frames
            return AnimatedQRCodeViewModel(qrCodes: qrCodes.map(\.payload))
        }, completion: completion)
    }

    func exportRoot(
        seedName: String,
        _ completion: @escaping (Result<AnimatedQRCodeViewModel, ServiceError>) -> Void
    ) {
        backendService.performCall({
            let qrCodes = try exportKeyInfo(seedName: seedName, exportedSet: .selected(s: [])).frames
            return AnimatedQRCodeViewModel(qrCodes: qrCodes.map(\.payload))
        }, completion: completion)
    }
}
