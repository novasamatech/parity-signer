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
        switch (data.actionResult.screen) {
        case .Scan :
            TransactionScreen()
        case .Keys(let value):
            KeyManager(content: value)
        case .Settings(let value) :
            SettingsScreen(content: value)
        case .Log(let value) :
            HistoryScreen(content: value)
        case .LogDetails(let value):
            EventDetails(content: value)
        case .Transaction(let value):
            TransactionPreview(content: value)
        case .SeedSelector(let value):
            SeedManager(content: value)
        case .KeyDetails(let value):
            ExportAddress(content: value)
        case .NewSeed(let value):
            NewSeedScreen(content: value)
        case .RecoverSeedName(let value):
            RecoverSeedName(content: value)
        case .RecoverSeedPhrase(let value):
            RecoverSeedPhrase(content: value)
        case .DeriveKey(let value):
            NewAddressScreen(content: value)
        case .Verifier(let value):
            VerifierScreen(content: value)
        case .ManageNetworks(let value):
            ManageNetworks(content: value)
        case .NetworkDetails(let value):
            NetworkDetails(content: value)
        case .SignSufficientCrypto(let value):
            SignSufficientCrypto(content: value)
        case .SelectSeedForBackup(let value):
            SelectSeedForBackup(content: value)
        case .Documents:
            DocumentModal()
        case .KeyDetailsMulti(let value):
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
