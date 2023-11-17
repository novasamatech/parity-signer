//
//  ScanTabService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 08/05/2023.
//

import Foundation

final class ScanTabService {
    private let backendService: BackendService

    init(
        backendService: BackendService = BackendService()
    ) {
        self.backendService = backendService
    }

    func continueTransactionSigning(_ seedNames: [String], _ seedPhrasesDictionary: [String: String], completion: (Result<ActionResult, TransactionError>) -> Void) {
//        backendService.performCall {
//
//        } completion: { result in
//
//        }
    }

    func performTransaction(with payload: String, completion: (Result<ActionResult, TransactionError>) -> Void) {
//        backendService.performCall {
//
//        } completion: { result in
//
//        }
    }
}

private extension ScanTabService {
    func formattedPhrase(seedNames: [String], with dictionary: [String: String]) -> String {
        seedNames.reduce(into: "") { $0 += "\(dictionary[$1] ?? "")\n" }
    }
}
