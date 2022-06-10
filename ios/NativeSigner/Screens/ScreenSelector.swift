//
//  ScreenSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 26.11.2021.
//

import SwiftUI

struct ScreenSelector: View {
    let screenData: ScreenData
    let pushButton: (Action, String, String) -> Void
    let getSeed: (String) -> String
    let doJailbreak: () -> Void
    let pathCheck: (String, String, String) -> DerivationCheck
    let createAddress: (String, String) -> Void
    let checkSeedCollision: (String) -> Bool
    let restoreSeed: (String, String, Bool) -> Void
    
    var body: some View {
        switch (screenData) {
        case .scan :
            TransactionScreen()
        case .keys(let value):
            KeyManager(content: value)
        case .settings(let value) :
            SettingsScreen(content: value)
        case .log(let value) :
            HistoryScreen(content: value)
        case .logDetails(let value):
            EventDetails(content: value)
        case .transaction(let value):
            TransactionPreview(
                content: value,
                pushButton: pushButton
            )
        case .seedSelector(let value):
            SeedManager(content: value)
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
            ManageNetworks(content: value)
        case .nNetworkDetails(let value):
            NetworkDetails(content: value)
        case .signSufficientCrypto(let value):
            SignSufficientCrypto(
                content: value,
                pushButton: pushButton,
                getSeed: getSeed)
        case .selectSeedForBackup(let value):
            SelectSeedForBackup(content: value)
        case .documents:
            DocumentModal()
        case .keyDetailsMulti(let value):
            KeyDetailsMulti(
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
