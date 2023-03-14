//
//  ModalFactory.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 10/03/2023.
//

import SwiftUI

final class ModalFactory {
    @ViewBuilder
    func modal(for modalData: ModalData?) -> some View {
        switch modalData {
        case let .sufficientCryptoReady(value):
            SufficientCryptoReady(content: value)
        case let .newSeedBackup(value):
            CreateKeySetSeedPhraseView(viewModel: .init(dataModel: value))
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
