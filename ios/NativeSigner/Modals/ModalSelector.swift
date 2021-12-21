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
        switch (data.actionResult.modal) {
        case .Empty:
            EmptyView()
        case .NewSeedMenu:
            NewSeedMenu()
        case .NetworkMenu(let value):
            NetworkManager(content: value)
        case .SeedMenu:
            SeedMenu()
        case .Backup(let value):
            Backup(content: value)
        case .PasswordConfirm(let value):
            PasswordConfirm(content: value)
        case .SignatureReady(let value):
            SignatureReady(content: value)
        case .EnterPassword(let value):
            EnterPassword(content: value)
        case .LogRight:
            LogMenu()
        case .NetworkDetailsMenu:
            NetworkDetailsMenu()
        case .ManageMetadata(let value):
            ManageMetadata(content: value)
        case .SufficientCryptoReady(let value):
            SufficientCryptoReady(content: value)
        case .KeyDetailsAction:
            KeyMenu()
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
