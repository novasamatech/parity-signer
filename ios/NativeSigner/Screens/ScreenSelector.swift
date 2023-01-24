//
//  ScreenSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 26.11.2021.
//

import SwiftUI

struct ScreenSelector: View {
    @EnvironmentObject private var data: SignerDataModel
    @EnvironmentObject var navigation: NavigationCoordinator

    var body: some View {
        switch navigation.actionResult.screenData {
        case let .keys(keyName):
            KeyDetailsView(
                viewModel: .init(
                    keyName: keyName
                ),
                forgetKeyActionHandler: ForgetKeySetAction(navigation: navigation),
                resetWarningAction: ResetConnectivtyWarningsAction(alert: $data.alert)
            )
        case .settings:
            SettingsView(viewModel: .init())
        case let .log(value):
            LogsListView(viewModel: .init(logs: value))
        case let .logDetails(value):
            EventDetails(content: value)
        case let .seedSelector(value):
            KeySetList(viewModel: .init(), dataModel: .constant(value))
        case let .keyDetails(value):
            if let value = value {
                KeyDetailsPublicKeyView(
                    forgetKeyActionHandler: ForgetSingleKeyAction(navigation: navigation),
                    viewModel: KeyDetailsPublicKeyViewModel(value),
                    actionModel: KeyDetailsPublicKeyActionModel(value),
                    exportPrivateKeyService: ExportPrivateKeyService(keyDetails: value),
                    resetWarningAction: ResetConnectivtyWarningsAction(alert: $data.alert)
                )
            } else {
                EmptyView()
            }
        case let .newSeed(value):
            NewSeedScreen(
                content: value
            )
        case let .recoverSeedName(value):
            RecoverSeedName(
                content: value
            )
        case let .recoverSeedPhrase(value):
            RecoverSeedPhrase(
                content: value
            )
        case let .deriveKey:
            CreateDerivedKeyView(viewModel: .init())
        case let .vVerifier(value):
            VerfierCertificateView(viewModel: .init(content: value))
        case let .manageNetworks(value):
            NetworkSelectionSettings(viewModel: .init(networks: value.networks))
        case let .nNetworkDetails(value):
            NetworkSettingsDetails(viewModel: .init(networkDetails: value))
        case let .signSufficientCrypto(value):
            SignSufficientCrypto(
                content: value
            )
        case let .selectSeedForBackup(value):
            SelectSeedForBackup(
                content: value
            )
        case .documents:
            OnboardingAgreementsView(viewModel: .init())
        // Screens handled outside of Rust navigation
        case .scan:
            EmptyView()
        case .transaction:
            EmptyView()
        case .keyDetailsMulti:
            EmptyView()
        }
    }
}

// struct ScreenSelector_Previews: PreviewProvider {
//    static var previews: some View {
//        ScreenSelector()
//    }
// }
