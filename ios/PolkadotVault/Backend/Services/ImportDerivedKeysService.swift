//
//  ImportDerivedKeysService.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 02/01/2023.
//

import Foundation

// sourcery: AutoMockable
protocol ImportDerivedKeysServicing: AnyObject {
    func importDerivedKeys(
        _ seedPreviews: [SeedKeysPreview],
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    )
    func updateWithSeeds(
        _ seedPreviews: [SeedKeysPreview],
        completion: @escaping (Result<[SeedKeysPreview], ServiceError>) -> Void
    )
}

extension ImportDerivedKeysService: ImportDerivedKeysServicing {}

final class ImportDerivedKeysService {
    private let seedsMediator: SeedsMediating
    private let backendService: BackendService

    init(
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
        backendService: BackendService = BackendService()
    ) {
        self.seedsMediator = seedsMediator
        self.backendService = backendService
    }

    func importDerivedKeys(
        _ seedPreviews: [SeedKeysPreview],
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        var seedPreviewsToImport = seedPreviews
        for (index, seedPreview) in seedPreviewsToImport.enumerated() {
            seedPreviewsToImport[index].derivedKeys = seedPreview.derivedKeys
                .filter { $0.status == .importable }
        }
        backendService.performCall({
            try importDerivations(seedDerivedKeys: seedPreviewsToImport)
        }, completion: completion)
    }

    func updateWithSeeds(
        _ seedPreviews: [SeedKeysPreview],
        completion: @escaping (Result<[SeedKeysPreview], ServiceError>) -> Void
    ) {
        let seeds = seedsMediator.getAllSeeds()
        backendService.performCall({
            try populateDerivationsHasPwd(
                seeds: seeds,
                seedDerivedKeys: seedPreviews
            )
        }, completion: completion)
    }
}
