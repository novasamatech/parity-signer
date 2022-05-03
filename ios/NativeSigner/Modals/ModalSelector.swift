//
//  ModalSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.12.2021.
//

import SwiftUI

struct ModalSelector: View {
    @EnvironmentObject var data: SignerDataModel
    
    var body: some View {
        switch (data.actionResult.modalData) {
        case .newSeedMenu:
            NewSeedMenu()
        case .networkSelector(let value):
            NetworkManager(content: value)
        case .seedMenu(let value):
            SeedMenu(content: value)
        case .backup(let value):
            Backup(content: value)
        case .passwordConfirm(let value):
            PasswordConfirm(content: value)
        case .signatureReady(let value):
            SignatureReady(content: value)
        case .enterPassword(let value):
            EnterPassword(content: value)
        case .logRight(let value):
            LogMenu(content: value)
        case .networkDetailsMenu:
            NetworkDetailsMenu()
        case .manageMetadata://(let value):
            ManageMetadata()//content: value)
        case .sufficientCryptoReady(let value):
            SufficientCryptoReady(content: value)
        case .keyDetailsAction:
            KeyMenu()
        case .typesInfo(let value):
            TypesMenu(content: value)
        case .newSeedBackup(let value):
            NewSeedBackupModal(content: value)
        case .logComment:
            LogComment()
        case .selectSeed://(let value):
            EmptyView()//SelectSeed(content: value)
        case .manageNetworks(_):
            EmptyView()
        case nil:
            EmptyView()
        }
    }
}

/*
 struct ModalSelector_Previews: PreviewProvider {
 static var previews: some View {
 ModalSelector()
 }
 }
 */
