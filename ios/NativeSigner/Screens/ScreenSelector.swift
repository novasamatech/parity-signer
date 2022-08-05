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
        case .scan :
            TransactionScreen(
                pushButton: pushButton
            )
        case .keys(let value):
            KeyManager(
                content: value,
                alert: alert,
                alertShow: alertShow,
                increment: increment,
                pushButton: pushButton
            )
        case .settings(let value) :
            SettingsScreen(
                content: value,
                appVersion: appVersion,
                doWipe: doWipe,
                pushButton: pushButton
            )
        case .log(let value) :
            HistoryScreen(
                content: value,
                pushButton: pushButton
            )
        case .logDetails(let value):
            EventDetails(content: value)
        case .transaction(let value):
            TransactionPreview(
                content: value,
                sign: sign,
                pushButton: pushButton
            )
        case .seedSelector(let value):
            SeedManager(
                content: value,
                pushButton: pushButton
            )
        case .keyDetails(let value):
            ExportAddress(content: value)
        case .newSeed(let value):
            NewSeedScreen(
                content: value,
                checkSeedCollision: checkSeedCollision,
                pushButton: pushButton
            )
        case .recoverSeedName(let value):
            RecoverSeedName(
                content: value,
                checkSeedCollision: checkSeedCollision,
                pushButton: pushButton
            )
        case .recoverSeedPhrase(let value):
            RecoverSeedPhrase(
                content: value,
                restoreSeed: restoreSeed,
                pushButton: pushButton
            )
        case .deriveKey(let value):
            NewAddressScreen(
                content: value,
                pathCheck: pathCheck,
                createAddress: createAddress,
                pushButton: pushButton
            )
        case .vVerifier(let value):
            VerifierScreen(
                content: value,
                doJailbreak: doJailbreak
            )
        case .manageNetworks(let value):
            ManageNetworks(
                content: value,
                pushButton: pushButton
            )
        case .nNetworkDetails(let value):
            NetworkDetails(
                content: value,
                pushButton: pushButton
            )
        case .signSufficientCrypto(let value):
            SignSufficientCrypto(
                content: value,
                pushButton: pushButton,
                getSeed: getSeed)
        case .selectSeedForBackup(let value):
            SelectSeedForBackup(
                content: value,
                pushButton: pushButton
            )
        case .documents:
            DocumentModal()
        case .keyDetailsMulti(let value):
            KeyDetailsMulti(
                content: value,
                pushButton: pushButton
            )
        case .signatureReady(let value):
            SignatureReady(
                content: value,
                pushButton: pushButton
            )
        }
    }
}

/*
 struct ScreenSelector_Previews: PreviewProvider {
 static var previews: some View {
 ScreenSelector()
 }
 }
 */
