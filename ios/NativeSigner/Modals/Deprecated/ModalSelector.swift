//
//  ModalSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.12.2021.
//

import SwiftUI

struct ModalSelector: View {
    @EnvironmentObject private var data: SignerDataModel
    @EnvironmentObject private var navigation: NavigationCoordinator

    var body: some View {
        switch navigation.actionResult.modalData {
        case let .sufficientCryptoReady(value):
            SufficientCryptoReady(content: value)
        case let .newSeedBackup(value):
            NewSeedBackupModal(
                content: value
            )
        case let .selectSeed(value):
            SelectSeed(
                content: value
            )
        // Handled in native navigation
        case
            .typesInfo,
            .passwordConfirm,
            .logComment,
            .enterPassword,
            .backup,
            .keyDetailsAction,
            .newSeedMenu,
            .seedMenu,
            .signatureReady,
            .logRight,
            .networkSelector,
            .manageMetadata,
            .networkDetailsMenu,
            nil:
            EmptyView()
        }
    }
}

// struct ModalSelector_Previews: PreviewProvider {
// static var previews: some View {
// ModalSelector()
// }
// }
