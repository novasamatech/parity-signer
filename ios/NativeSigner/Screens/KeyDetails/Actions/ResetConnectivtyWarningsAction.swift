//
//  ResetConnectivtyWarningsAction.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 29/09/2022.
//

import SwiftUI

final class ResetConnectivtyWarningsAction {
    private let databaseMediator: DatabaseMediating
    @Binding var alert: Bool

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator(),
        alert: Binding<Bool>
    ) {
        self.databaseMediator = databaseMediator
        _alert = alert
    }

    func resetConnectivityWarnings() {
        try? historyAcknowledgeWarnings()
        _ = try? historyGetWarnings()
        alert = false
    }
}
