//
//  ModalSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.12.2021.
//

import SwiftUI

struct ModalSelector: View {
    let modalData: ModalData?
    let alert: Bool
    let alertShow: () -> Void
    let pushButton: (Action, String, String) -> Void
    let removeSeed: (String) -> Void
    let restoreSeed: (String, String, Bool) -> Void
    let createAddress: (String, String) -> Void
    let getSeedForBackup: (String) -> String
    let sign: (String, String) -> Void

    var body: some View {
        switch modalData {
        case .newSeedMenu:
            NewSeedMenu(
                alert: alert,
                alertShow: alertShow,
                pushButton: pushButton
            )
        case .networkSelector(let value):
            NetworkManager(
                content: value,
                pushButton: pushButton
            )
        case .seedMenu(let value):
            SeedMenu(
                content: value,
                alert: alert,
                alertShow: alertShow,
                removeSeed: removeSeed,
                pushButton: pushButton
            )
        case .backup(let value):
            Backup(content: value,
                   alert: alert,
                   getSeedForBackup: getSeedForBackup,
                   pushButton: pushButton)
        case .passwordConfirm(let value):
            PasswordConfirm(content: value,
                            createAddress: createAddress)
        case .enterPassword(let value):
            EnterPassword(content: value,
                          pushButton: pushButton)
        case .logRight(let value):
            LogMenu(content: value,
                    pushButton: pushButton)
        case .networkDetailsMenu:
            NetworkDetailsMenu(
                               pushButton: pushButton)
        case .manageMetadata(let value):
            ManageMetadata(content: value,
                           pushButton: pushButton)
        case .sufficientCryptoReady(let value):
            SufficientCryptoReady(content: value)
        case .keyDetailsAction:
            KeyMenu(
                    pushButton: pushButton)
        case .typesInfo(let value):
            TypesMenu(content: value,
                      pushButton: pushButton)
        case .newSeedBackup(let value):
            NewSeedBackupModal(content: value,
                               restoreSeed: restoreSeed,
                               pushButton: pushButton)
        case .logComment:
            LogComment(
                       pushButton: pushButton)
        case .selectSeed(let value):
            SelectSeed(
                content: value,
                sign: sign,
                pushButton: pushButton
            )
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
