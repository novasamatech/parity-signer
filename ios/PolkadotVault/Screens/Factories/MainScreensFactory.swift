//
//  MainScreensFactory.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 14/03/2023.
//

import SwiftUI

final class MainScreensFactory {
    @ViewBuilder
    func screen(for screenData: ScreenData) -> some View {
        switch screenData {
        case let .seedSelector(value):
            KeySetList(viewModel: .init(dataModel: value))
        case .settings:
            SettingsView(viewModel: .init())
        case .deriveKey:
            CreateDerivedKeyView(viewModel: .init())
        // Screens handled outside of Rust navigation
        case .documents,
             .selectSeedForBackup,
             .newSeed,
             .recoverSeedName,
             .recoverSeedPhrase,
             .vVerifier,
             .scan,
             .transaction,
             .keyDetailsMulti,
             .manageNetworks,
             .nNetworkDetails,
             .logDetails,
             .signSufficientCrypto,
             .keys,
             .keyDetails,
             .log:
            EmptyView()
        }
    }
}
