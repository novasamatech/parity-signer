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
    let navigationRequest: NavigationRequest
    let removeSeed: (String) -> Void
    let restoreSeed: (String, String, Bool) -> Void
    let createAddress: (String, String) -> Void
    let getSeedForBackup: (String) -> String
    let sign: (String, String) -> Void

    var body: some View {
        switch modalData {
        case let .networkSelector(value):
            NetworkManager(
                content: value,
                navigationRequest: navigationRequest
            )
        case let .seedMenu(value):
            SeedMenu(
                content: value,
                alert: alert,
                alertShow: alertShow,
                removeSeed: removeSeed,
                navigationRequest: navigationRequest
            )
        case let .backup(value):
            Backup(
                content: value,
                alert: alert,
                getSeedForBackup: getSeedForBackup,
                navigationRequest: navigationRequest
            )
        case let .passwordConfirm(value):
            PasswordConfirm(
                content: value,
                createAddress: createAddress
            )
        case let .signatureReady(value):
            SignatureReady(
                content: value,
                navigationRequest: navigationRequest
            )
        case let .enterPassword(value):
            EnterPassword(
                content: value,
                navigationRequest: navigationRequest
            )
        case let .logRight(value):
            LogMenu(
                content: value,
                navigationRequest: navigationRequest
            )
        case .networkDetailsMenu:
            NetworkDetailsMenu(
                navigationRequest: navigationRequest
            )
        case let .manageMetadata(value):
            ManageMetadata(
                content: value,
                navigationRequest: navigationRequest
            )
        case let .sufficientCryptoReady(value):
            SufficientCryptoReady(content: value)
        case .keyDetailsAction:
            KeyMenu(
                navigationRequest: navigationRequest
            )
        case let .typesInfo(value):
            TypesMenu(
                content: value,
                navigationRequest: navigationRequest
            )
        case let .newSeedBackup(value):
            NewSeedBackupModal(
                content: value,
                restoreSeed: restoreSeed,
                navigationRequest: navigationRequest
            )
        case .logComment:
            LogComment(
                navigationRequest: navigationRequest
            )
        case let .selectSeed(value):
            SelectSeed(
                content: value,
                sign: sign,
                navigationRequest: navigationRequest
            )
        case .newSeedMenu:
            EmptyView()
        case nil:
            EmptyView()
        }
    }
}

// struct ModalSelector_Previews: PreviewProvider {
// static var previews: some View {
// ModalSelector()
// }
// }
