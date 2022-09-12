//
//  KeyMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 21.12.2021.
//

import SwiftUI
//
// final class KeyMenuActionModel: ObservableObject {
//    var networkInfo:
// }

struct KeyMenu: View {
    @State private var removeConfirm = false
    @State private var shouldPresentExportKeysModal = false
    @State private var isPresentingExportKeysWarningModal = false
    @State private var isPresentingExportKeysModal = false
    let exportKeyService: ExportPrivateKeyService
    @ObservedObject var navigation: NavigationCoordinator

    var body: some View {
        MenuStack {
            HeaderBar(line1: Localizable.keyMenu.key, line2: Localizable.selectAction.key).padding(.top, 10)
            MenuButtonsStack {
                // Don't show `Export Private Key` if intermediate state is broken or when key is password protected
                if let currentKeyDetails = navigation.currentKeyDetails,
                   currentKeyDetails.address.hasPwd == false {
                    BigButton(
                        text: Localizable.KeyScreen.Action.export.key,
                        isShaded: false,
                        isDangerous: false,
                        action: {
                            isPresentingExportKeysWarningModal = true
                        }
                    )
                }
                BigButton(
                    text: Localizable.forgetThisKeyForever.key,
                    isShaded: true,
                    isDangerous: true,
                    action: {
                        removeConfirm.toggle()
                    }
                )
            }
        }
        .alert(isPresented: $removeConfirm, content: {
            Alert(
                title: Localizable.forgetThisKey.text,
                message: Localizable.ThisKeyWillBeRemovedForThisNetwork.areYouSure.text,
                primaryButton: .cancel(Localizable.cancel.text),
                secondaryButton: .destructive(
                    Localizable.removeKey.text,
                    action: { navigation.perform(navigation: .init(action: .removeKey)) }
                )
            )
        })
        .fullScreenCover(
            isPresented: $isPresentingExportKeysWarningModal,
            onDismiss: {
                if shouldPresentExportKeysModal {
                    shouldPresentExportKeysModal.toggle()
                    isPresentingExportKeysModal.toggle()
                } else {
                    // If user cancelled, mimic Rust state machine and hide "..." modal menu
                    navigation.perform(navigation: .init(action: .rightButtonAction))
                }
            }
        ) {
            ExportPrivateKeyWarningModal(
                isPresentingExportKeysWarningModal: $isPresentingExportKeysWarningModal,
                shouldPresentExportKeysModal: $shouldPresentExportKeysModal
            )
            .clearModalBackground()
        }
        .fullScreenCover(
            isPresented: $isPresentingExportKeysModal,
            onDismiss: {
                // When user finished Export Private Key interaction, mimic Rust state machine and hide "..." modal menu
                navigation.perform(navigation: .init(action: .rightButtonAction))
            }
        ) {
            ExportPrivateKeyModal(
                isPresentingExportKeysModal: $isPresentingExportKeysModal,
                navigation: navigation,
                viewModel: exportKeyService.exportPrivateKey(from: navigation.currentKeyDetails)
            )
            .clearModalBackground()
        }
    }
}

// struct KeyMenu_Previews: PreviewProvider {
// static var previews: some View {
// KeyMenu()
// }
// }
