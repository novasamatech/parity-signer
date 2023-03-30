//
//  MainScreensFactory.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 14/03/2023.
//

import SwiftUI

final class MainScreensFactory {
    @ViewBuilder
    // swiftlint:disable function_body_length
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
        case .log:
            LogsListView(viewModel: .init())
        case let .keyDetails(value):
            if let value = value {
                KeyDetailsPublicKeyView(
                    viewModel: KeyDetailsPublicKeyViewModel(value),
                    actionModel: KeyDetailsPublicKeyActionModel(value),
                    exportPrivateKeyService: ExportPrivateKeyService(keyDetails: value)
                )
            } else {
                EmptyView()
            }
        case .newSeed:
            EnterKeySetNameView(viewModel: .init())
        case let .recoverSeedName(value):
            RecoverKeySetNameView(viewModel: .init(content: value))
        case let .recoverSeedPhrase(value):
            RecoverKeySetSeedPhraseView(viewModel: .init(content: value))
        case .deriveKey:
            CreateDerivedKeyView(viewModel: .init())
        case let .vVerifier(value):
            VerfierCertificateView(viewModel: .init(content: value))
        case let .manageNetworks(value):
            NetworkSelectionSettings(viewModel: .init(networks: value.networks))
        case let .nNetworkDetails(value):
            NetworkSettingsDetails(viewModel: .init(networkDetails: value))
        case let .signSufficientCrypto(value):
            SignSpecsListView(viewModel: .init(content: value))
        // Screens handled outside of Rust navigation
        case .documents,
             .selectSeedForBackup,
             .scan,
             .transaction,
             .keyDetailsMulti,
             .logDetails:
            EmptyView()
        }
    }
}
