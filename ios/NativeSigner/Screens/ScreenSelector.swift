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
    let screenData: ScreenData
    let navigationRequest: NavigationRequest
    let getSeed: (String) -> String
    let doJailbreak: () -> Void
    let pathCheck: (String, String, String) -> DerivationCheck
    let createAddress: (String, String) -> Void
    let checkSeedCollision: (String) -> Bool
    let restoreSeed: (String, String, Bool) -> Void
    let sign: (String, String) -> Void
    let doWipe: () -> Void
    let alertShow: () -> Void
    let increment: (String, String) -> Void

    var body: some View {
        switch screenData {
        case .scan:
            TransactionScreen(
                navigationRequest: navigationRequest
            )
        case let .keys(value):
            KeyDetailsView(
                forgetKeyActionHandler: ForgetKeySetAction(),
                viewModel: KeyDetailsViewModel(value),
                actionModel: KeyDetailsActionModel(value, alert: data.alert, alertShow: alertShow),
                exportPrivateKeyService: PrivateKeyQRCodeService(navigation: navigation, keys: value),
                resetWarningAction: ResetConnectivtyWarningsAction(alert: $data.alert)
            )
        case let .settings(value):
            SettingsScreen(
                content: value,
                doWipe: doWipe,
                navigationRequest: navigationRequest
            )
        case let .log(value):
            HistoryScreen(
                content: value,
                navigationRequest: navigationRequest
            )
        case let .logDetails(value):
            EventDetails(content: value)
        case let .transaction(value):
            TransactionPreview(
                content: value,
                sign: sign,
                navigationRequest: navigationRequest
            )
        case let .seedSelector(value):
            KeySetList(
                viewModel: KeySetListViewModelBuilder().build(for: value)
            )
        case let .keyDetails(value):
            KeyDetailsPublicKeyView(
                forgetKeyActionHandler: ForgetSingleKeyAction(),
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
            VerifierScreen(
                content: value,
                doJailbreak: doJailbreak
            )
        case let .manageNetworks(value):
            ManageNetworks(
                content: value,
                navigationRequest: navigationRequest
            )
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
        }
    }
}

// struct ScreenSelector_Previews: PreviewProvider {
//    static var previews: some View {
//        ScreenSelector()
//    }
// }
