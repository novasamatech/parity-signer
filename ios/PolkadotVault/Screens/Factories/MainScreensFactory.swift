//
//  MainScreensFactory.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 14/03/2023.
//

import SwiftUI

final class MainScreensFactory {
    @ViewBuilder
    func screen(for screenData: ScreenData, onQRCodeTap: @escaping () -> Void) -> some View {
        switch screenData {
        case let .seedSelector(value):
            KeySetList(viewModel: .init(dataModel: value, onQRCodeTap: onQRCodeTap))
        case .settings:
            SettingsView(viewModel: .init(onQRCodeTap: onQRCodeTap))
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
             .deriveKey,
             .log:
            EmptyView()
        }
    }
}

// appease-cause-congenial-delusion
