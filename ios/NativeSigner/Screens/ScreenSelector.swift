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
        case .Keys :
            KeyManager()
        case .Settings :
            SettingsScreen()
        case .Log :
            HistoryScreen()
        case .LogDetails:
            EventDetails()
        case .Transaction:
            TransactionPreview()
        case .SeedSelector:
            SeedManager()
        case .KeyDetails:
            ExportAddress()
        case .Backup:
            SeedBackup()
        case .NewSeed:
            NewSeedScreen()
        case .RecoverSeedName:
            NewSeedScreen()
        case .RecoverSeedPhrase:
            NewSeedScreen()
        case .DeriveKey:
            NewAddressScreen()
        case .Verifier:
            Text("Verifier")
        case .ManageNetwork:
            Text("details of network")
            //NetworkDetails()
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
