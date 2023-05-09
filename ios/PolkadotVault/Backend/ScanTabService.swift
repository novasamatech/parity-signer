//
//  ScanTabService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 08/05/2023.
//

import Foundation

final class ScanTabService {
    private let navigation: NavigationCoordinator
    private let backendActionPerformer: BackendNavigationPerforming

    init(
        navigation: NavigationCoordinator = NavigationCoordinator(),
        backendActionPerformer: BackendNavigationPerforming = BackendNavigationAdapter()
    ) {
        self.navigation = navigation
        self.backendActionPerformer = backendActionPerformer
    }

    func startQRScan() {
        navigation.performFake(navigation: .init(action: .start))
        navigation.performFake(navigation: .init(action: .navbarScan))
    }

    func continueTransactionSigning(_ seedNames: [String], _ seedPhrasesDictionary: [String: String]) -> ActionResult? {
        navigation.performFake(
            navigation:
            .init(
                action: .goForward,
                details: "",
                seedPhrase: formattedPhrase(seedNames: seedNames, with: seedPhrasesDictionary)
            )
        )
    }

    func performTransaction(with payload: String) -> Result<ActionResult, TransactionError> {
        backendActionPerformer.performTransaction(with: payload)
    }

    func resetNavigationState() {
        startQRScan()
    }

    func onTransactionApprove() {
        navigation.performFake(navigation: .init(action: .goForward))
        startQRScan()
    }

    func attemptPassword(_ password: String) -> ActionResult? {
        navigation.performFake(navigation: .init(action: .goForward, details: password))
    }

    func onSuccessfullPasswordEntry() {
        navigation.performFake(navigation: .init(action: .goBack))
    }
}

private extension ScanTabService {
    func formattedPhrase(seedNames: [String], with dictionary: [String: String]) -> String {
        seedNames.reduce(into: "") { $0 += "\(dictionary[$1] ?? "")\n" }
    }
}
