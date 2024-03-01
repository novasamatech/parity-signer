//
//  BananaSplitService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 09/01/2024.
//

import Foundation

struct BananaSplitBackup: Equatable, Codable {
    let qrCodes: [[UInt8]]
}

// sourcery: AutoMockable
protocol BananaSplitServicing: AnyObject {
    func encrypt(
        secret: String,
        title: String,
        passphrase: String,
        totalShards: UInt32,
        requiredShards: UInt32,
        _ completion: @escaping (Result<BananaSplitBackup, ServiceError>) -> Void
    )
    func generatePassphrase(
        with words: UInt32,
        _ completion: @escaping (Result<String, ServiceError>) -> Void
    )
}

extension BananaSplitService: BananaSplitServicing {}

final class BananaSplitService {
    private let backendService: BackendService

    init(
        backendService: BackendService = BackendService()
    ) {
        self.backendService = backendService
    }

    func encrypt(
        secret: String,
        title: String,
        passphrase: String,
        totalShards: UInt32,
        requiredShards: UInt32,
        _ completion: @escaping (Result<BananaSplitBackup, ServiceError>) -> Void
    ) {
        backendService.performCall({
            let qrCodes = try bsEncrypt(
                secret: secret,
                title: title,
                passphrase: passphrase,
                totalShards: totalShards,
                requiredShards: requiredShards
            )
            return BananaSplitBackup(qrCodes: qrCodes.map(\.payload))
        }, completion: completion)
    }

    func generatePassphrase(
        with words: UInt32,
        _ completion: @escaping (Result<String, ServiceError>) -> Void
    ) {
        backendService.performCall({
            bsGeneratePassphrase(n: words)
        }, completion: completion)
    }
}
