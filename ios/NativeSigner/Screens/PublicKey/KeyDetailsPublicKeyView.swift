//
//  KeyDetailsPublicKeyView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 13/09/2022.
//

import SwiftUI

struct KeyDetailsPublicKeyView: View {
    private let viewModel: KeyDetailsPublicKeyViewModel
    private let actionModel: KeyDetailsPublicKeyActionModel
    private let exportKeyService: ExportPrivateKeyService
    private let forgetKeyActionHandler: ForgetSingleKeyAction

    @State private var isShowingRemoveConfirmation = false
    @State private var isShowingActionSheet = false
    @State private var isPresentingExportKeysWarningModal = false
    @State private var isPresentingExportKeysModal = false

    @State private var shouldPresentExportKeysWarningModal = false
    @State private var shouldPresentExportKeysModal = false
    @State private var shouldPresentRemoveConfirmationModal = false

    @ObservedObject private var navigation: NavigationCoordinator

    init(
        navigation: NavigationCoordinator,
        forgetKeyActionHandler: ForgetSingleKeyAction,
        viewModel: KeyDetailsPublicKeyViewModel,
        actionModel: KeyDetailsPublicKeyActionModel,
        exportKeyService: ExportPrivateKeyService = ExportPrivateKeyService()
    ) {
        self.navigation = navigation
        self.forgetKeyActionHandler = forgetKeyActionHandler
        self.viewModel = viewModel
        self.actionModel = actionModel
        self.exportKeyService = exportKeyService
    }

    var body: some View {
        VStack(spacing: 0) {
            // Navigation bar
            NavigationBarView(
                navigation: navigation,
                viewModel: .init(
                    title: Localizable.PublicKeyDetails.Label.title.string,
                    subtitle: viewModel.isRootKey ? nil : Localizable.PublicKeyDetails.Label.subtitle.string,
                    leftButton: .xmark,
                    rightButton: .more
                ),
                actionModel: .init(rightBarMenuAction: { isShowingActionSheet.toggle() })
            )
            ScrollView {
                VStack {
                    VStack(spacing: 0) {
                        QRCodeContainerView(viewModel: viewModel.qrCode)
                        if let addressFooter = viewModel.addressFooter {
                            QRCodeAddressFooterView(viewModel: addressFooter)
                        }
                        if let rootFooter = viewModel.rootFooter {
                            QRCodeRootFooterView(viewModel: rootFooter)
                        }
                    }
                    .background(
                        RoundedRectangle(cornerRadius: CornerRadius.medium)
                            .stroke(Asset.fill12.swiftUIColor, lineWidth: 1)
                            .background(Asset.fill6.swiftUIColor)
                            .cornerRadius(CornerRadius.medium)
                    )
                    // Exposed key alert
                    if viewModel.isKeyExposed {
                        HStack {
                            Localizable.KeyScreen.Label.hotkey.text
                                .frame(maxWidth: .infinity, alignment: .leading)
                            Spacer().frame(maxWidth: Spacing.medium)
                            Asset.exclamationRed.swiftUIImage
                        }
                        .padding()
                        .foregroundColor(Asset.accentRed300.swiftUIColor)
                        .font(Fontstyle.bodyM.base)
                        .background(
                            RoundedRectangle(cornerRadius: CornerRadius.small)
                                .stroke(Asset.fill12.swiftUIColor, lineWidth: 1)
                                .background(Asset.accentRed300.swiftUIColor.opacity(0.12))
                                .cornerRadius(CornerRadius.small)
                        )
                    }
                }
                .padding([.leading, .trailing], Spacing.large)
                .padding([.top, .bottom], 60)
                .background(Asset.backgroundSolidSystem.swiftUIColor)
            }
            .background(Asset.backgroundSolidSystem.swiftUIColor)
        }
        // Action sheet
        .fullScreenCover(
            isPresented: $isShowingActionSheet,
            onDismiss: {
                if shouldPresentExportKeysWarningModal {
                    shouldPresentExportKeysWarningModal.toggle()
                    isPresentingExportKeysWarningModal.toggle()
                }
                if shouldPresentRemoveConfirmationModal {
                    shouldPresentRemoveConfirmationModal.toggle()
                    isShowingRemoveConfirmation.toggle()
                }
            }
        ) {
            PublicKeyActionsModal(
                shouldPresentExportKeysWarningModal: $shouldPresentExportKeysWarningModal,
                isShowingActionSheet: $isShowingActionSheet,
                shouldPresentRemoveConfirmationModal: $shouldPresentRemoveConfirmationModal,
                navigation: navigation
            )
            .clearModalBackground()
        }
        // Export private key warning
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
        // Export private key modal
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
        // Remove key modal
        .fullScreenCover(isPresented: $isShowingRemoveConfirmation) {
            HorizontalActionsBottomModal(
                viewModel: .forgetSingleKey,
                mainAction: forgetKeyActionHandler.forgetSingleKey(actionModel.removeSeed),
                // We need to fake right button action here or Rust machine will break
                // In old UI, if you dismiss equivalent of this modal, underlying modal would still be there,
                // so we need to inform Rust we actually hid it
                dismissAction: navigation.perform(navigation: .init(action: .rightButtonAction), skipDebounce: true),
                isShowingBottomAlert: $isShowingRemoveConfirmation
            )
            .clearModalBackground()
        }
    }
}

// struct KeyDetailsPublicKeyView_Previews: PreviewProvider {
//    static var previews: some View {
//        HStack {
//            VStack {
//                KeyDetailsPublicKeyView(
//                    navigation: NavigationCoordinator(),
//                    forgetKeyActionHandler: ForgetSingleKeyAction(navigation: NavigationCoordinator()),
//                    viewModel: PreviewData.exampleKeyDetailsPublicKey(),
//                    actionModel: KeyDetailsPublicKeyActionModel(removeSeed: "")
//                )
//            }
//            VStack {
//                KeyDetailsPublicKeyView(
//                    navigation: NavigationCoordinator(),
//                    forgetKeyActionHandler: ForgetSingleKeyAction(navigation: NavigationCoordinator()),
//                    viewModel: PreviewData.exampleKeyDetailsPublicKey(isKeyExposed: false),
//                    actionModel: KeyDetailsPublicKeyActionModel(removeSeed: "")
//                )
//            }
//            VStack {
//                KeyDetailsPublicKeyView(
//                    navigation: NavigationCoordinator(),
//                    forgetKeyActionHandler: ForgetSingleKeyAction(navigation: NavigationCoordinator()),
//                    viewModel: PreviewData.exampleKeyDetailsPublicKey(isRootKey: false),
//                    actionModel: KeyDetailsPublicKeyActionModel(removeSeed: "")
//                )
//            }
//            VStack {
//                KeyDetailsPublicKeyView(
//                    navigation: NavigationCoordinator(),
//                    forgetKeyActionHandler: ForgetSingleKeyAction(navigation: NavigationCoordinator()),
//                    viewModel: PreviewData.exampleKeyDetailsPublicKey(isKeyExposed: false, isRootKey: false),
//                    actionModel: KeyDetailsPublicKeyActionModel(removeSeed: "")
//                )
//            }
//        }
//        .previewLayout(.sizeThatFits)
//        .preferredColorScheme(.dark)
//    }
// }
