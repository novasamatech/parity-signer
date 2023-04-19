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
        case let .keys(keyName):
            KeyDetailsView(
                viewModel: .init(
                    keyName: keyName
                )
            )
        case .settings:
            SettingsView(viewModel: .init())
        case let .keyDetails(value):
            if let value = value {
                KeyDetailsPublicKeyView(viewModel: .init(keyDetails: value))
            } else {
                EmptyView()
            }
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
             .log:
            EmptyView()
        }
    }
}
