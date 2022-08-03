//
//  ScreenSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 26.11.2021.
//

import SwiftUI

struct ScreenSelector: View {
    let screenData: ScreenData
    let appVersion: String?
    let alert: Bool
    let pushButton: (Action, String, String) -> Void
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
                pushButton: pushButton
            )
        case let .keys(value):
            KeyManager(
                content: value,
                alert: alert,
                alertShow: alertShow,
                increment: increment,
                pushButton: pushButton
            )
        case let .settings(value):
            SettingsScreen(
                content: value,
                appVersion: appVersion,
                doWipe: doWipe,
                pushButton: pushButton
            )
        case let .log(value):
            HistoryScreen(
                content: value,
                pushButton: pushButton
            )
        case let .logDetails(value):
            EventDetails(content: value)
        case let .transaction(value):
            TransactionPreview(
                content: value,
                sign: sign,
                pushButton: pushButton
            )
        case let .seedSelector(value):
            SeedManager(
                content: value,
                pushButton: pushButton
            )
        case let .keyDetails(value):
            ExportAddress(content: value)
        case let .newSeed(value):
            NewSeedScreen(
                content: value,
                checkSeedCollision: checkSeedCollision,
                pushButton: pushButton
            )
        case let .recoverSeedName(value):
            RecoverSeedName(
                content: value,
                checkSeedCollision: checkSeedCollision,
                pushButton: pushButton
            )
        case let .recoverSeedPhrase(value):
            RecoverSeedPhrase(
                content: value,
                restoreSeed: restoreSeed,
                pushButton: pushButton
            )
        case let .deriveKey(value):
            NewAddressScreen(
                content: value,
                pathCheck: pathCheck,
                createAddress: createAddress,
                pushButton: pushButton
            )
        case let .vVerifier(value):
            VerifierScreen(
                content: value,
                doJailbreak: doJailbreak
            )
        case let .manageNetworks(value):
            ManageNetworks(
                content: value,
                pushButton: pushButton
            )
        case let .nNetworkDetails(value):
            NetworkDetails(
                content: value,
                pushButton: pushButton
            )
        case let .signSufficientCrypto(value):
            SignSufficientCrypto(
                content: value,
                pushButton: pushButton,
                getSeed: getSeed
            )
        case let .selectSeedForBackup(value):
            SelectSeedForBackup(
                content: value,
                pushButton: pushButton
            )
        case .documents:
            DocumentModal()
        case let .keyDetailsMulti(value):
            KeyDetailsMulti(
                content: value,
                pushButton: pushButton
            )
        }
    }
}

// struct ScreenSelector_Previews: PreviewProvider {
//    static var previews: some View {
//        ScreenSelector()
//    }
// }
