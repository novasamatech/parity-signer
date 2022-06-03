//
//  ScreenSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 26.11.2021.
//

import SwiftUI

struct ScreenSelector: View {
    @EnvironmentObject var data: SignerDataModel
    
    var body: some View {
        switch (data.actionResult.screenData) {
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
            TransactionPreview(content: value)
        case .seedSelector(let value):
            SeedManager(content: value)
        case .keyDetails(let value):
            ExportAddress(content: value)
        case .newSeed(let value):
            NewSeedScreen(content: value)
        case .recoverSeedName(let value):
            RecoverSeedName(content: value)
        case .recoverSeedPhrase(let value):
            RecoverSeedPhrase(content: value)
        case .deriveKey(let value):
            NewAddressScreen(content: value)
        case .vVerifier(let value):
            VerifierScreen(content: value)
        case .manageNetworks(let value):
            ManageNetworks(content: value)
        case .nNetworkDetails(let value):
            NetworkDetails(content: value)
        case .signSufficientCrypto(let value):
            SignSufficientCrypto(content: value)
        case .selectSeedForBackup(let value):
            SelectSeedForBackup(content: value)
        case .documents:
            DocumentModal()
        case .keyDetailsMulti(let value):
            KeyDetailsMulti(content: value)
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
