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
    @EnvironmentObject var appState: AppState

    let screenData: ScreenData
    let navigationRequest: NavigationRequest
    let getSeed: (String) -> String
    let pathCheck: (String, String, String) -> DerivationCheck
    let createAddress: (String, String) -> Void
    let checkSeedCollision: (String) -> Bool
    let restoreSeed: (String, String, Bool) -> Void
    let alertShow: () -> Void
    let increment: (String, String) -> Void

    var body: some View {
        switch screenData {
        case let .keys(value):
            KeyDetailsView(
                dataModel: KeyDetailsDataModel(value),
                viewModel: .init(
                    keysData: appState.userData.keysData,
                    exportPrivateKeyService: PrivateKeyQRCodeService(navigation: navigation, keys: value)
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
            KeySetList(
                viewModel: .init(listViewModel: KeySetListViewModelBuilder().build(for: value))
            )
        case let .keyDetails(value):
            KeyDetailsPublicKeyView(
                forgetKeyActionHandler: ForgetSingleKeyAction(navigation: navigation),
                viewModel: KeyDetailsPublicKeyViewModel(value),
                actionModel: KeyDetailsPublicKeyActionModel(value),
                exportPrivateKeyService: ExportPrivateKeyService(keyDetails: value),
                resetWarningAction: ResetConnectivtyWarningsAction(alert: $data.alert)
            )
        case let .newSeed(value):
            NewSeedScreen(
                content: value,
                checkSeedCollision: checkSeedCollision,
                navigationRequest: navigationRequest
            )
        case let .recoverSeedName(value):
            RecoverSeedName(
                content: value,
                checkSeedCollision: checkSeedCollision,
                navigationRequest: navigationRequest
            )
        case let .recoverSeedPhrase(value):
            RecoverSeedPhrase(
                content: value,
                restoreSeed: restoreSeed,
                navigationRequest: navigationRequest
            )
        case let .deriveKey(value):
            NewAddressScreen(
                content: value,
                pathCheck: pathCheck,
                createAddress: createAddress,
                navigationRequest: navigationRequest
            )
        case let .vVerifier(value):
            VerfierCertificateView(viewModel: .init(content: value))
        case let .manageNetworks(value):
            NetworkSelectionSettings(viewModel: .init(networks: value.networks))
        case let .nNetworkDetails(value):
            NetworkDetails(
                content: value,
                navigationRequest: navigationRequest
            )
        case let .signSufficientCrypto(value):
            SignSufficientCrypto(
                content: value,
                navigationRequest: navigationRequest,
                getSeed: getSeed
            )
        case let .selectSeedForBackup(value):
            SelectSeedForBackup(
                content: value,
                navigationRequest: navigationRequest
            )
        case .documents:
            DocumentModal()
        case let .keyDetailsMulti(value):
            KeyDetailsMulti(
                content: value,
                navigationRequest: navigationRequest
            )
        // Screens handled outside of Rust navigation
        case .scan:
            EmptyView()
        case .transaction:
            EmptyView()
        }
    }
}

// struct ScreenSelector_Previews: PreviewProvider {
//    static var previews: some View {
//        ScreenSelector()
//    }
// }
