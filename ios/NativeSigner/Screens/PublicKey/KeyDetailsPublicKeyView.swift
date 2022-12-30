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
    private let forgetKeyActionHandler: ForgetSingleKeyAction
    private let exportPrivateKeyService: ExportPrivateKeyService
    private let resetWarningAction: ResetConnectivtyWarningsAction

    // This view is recreated few times because of Rust navigation, for now we need to store modal view model in static
    // property because it can't be created earlier as it would trigger passcode request on the device
    private static var exportPrivateKeyViewModel: ExportPrivateKeyViewModel!

    @State private var isShowingRemoveConfirmation = false
    @State private var isShowingActionSheet = false
    @State private var isPresentingExportKeysWarningModal = false
    @State private var isPresentingExportKeysModal = false
    @State private var isPresentingConnectivityAlert = false

    @State private var shouldPresentExportKeysWarningModal = false
    @State private var shouldPresentExportKeysModal = false
    @State private var shouldPresentRemoveConfirmationModal = false

    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator
    @EnvironmentObject private var data: SignerDataModel

    init(
        forgetKeyActionHandler: ForgetSingleKeyAction,
        viewModel: KeyDetailsPublicKeyViewModel,
        actionModel: KeyDetailsPublicKeyActionModel,
        exportPrivateKeyService: ExportPrivateKeyService,
        resetWarningAction: ResetConnectivtyWarningsAction
    ) {
        self.forgetKeyActionHandler = forgetKeyActionHandler
        self.viewModel = viewModel
        self.actionModel = actionModel
        self.exportPrivateKeyService = exportPrivateKeyService
        self.resetWarningAction = resetWarningAction
    }

    var body: some View {
        VStack(spacing: 0) {
            // Navigation bar
            NavigationBarView(
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
                        AnimatedQRCodeView(
                            viewModel: Binding<AnimatedQRCodeViewModel>.constant(
                                .init(
                                    qrCodes: [viewModel.qrCode.payload]
                                )
                            )
                        )
                        .padding(0.5)
                        QRCodeAddressFooterView(viewModel: viewModel.footer)
                    }
                    .strokeContainerBackground()
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
                        .font(PrimaryFont.bodyM.font)
                        .strokeContainerBackground(CornerRadius.small, state: .error)
                    }
                }
                .padding([.leading, .trailing], Spacing.large)
                .padding([.top, .bottom], Spacing.componentSpacer)
                .background(Asset.backgroundPrimary.swiftUIColor)
            }
            .background(Asset.backgroundPrimary.swiftUIColor)
        }
        .onAppear {
            navigation.performFake(navigation: .init(action: .rightButtonAction))
        }
        // Action sheet
        .fullScreenCover(
            isPresented: $isShowingActionSheet,
            onDismiss: checkForActionsPresentation
        ) {
            PublicKeyActionsModal(
                shouldPresentExportKeysWarningModal: $shouldPresentExportKeysWarningModal,
                isShowingActionSheet: $isShowingActionSheet,
                shouldPresentRemoveConfirmationModal: $shouldPresentRemoveConfirmationModal
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
                viewModel: KeyDetailsPublicKeyView.exportPrivateKeyViewModel
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
                dismissAction: { _ = navigation.performFake(navigation: .init(action: .rightButtonAction)) }(),
                isShowingBottomAlert: $isShowingRemoveConfirmation
            )
            .clearModalBackground()
        }
        .fullScreenCover(
            isPresented: $isPresentingConnectivityAlert,
            onDismiss: checkForActionsPresentation
        ) {
            ErrorBottomModal(
                viewModel: connectivityMediator.isConnectivityOn ? .connectivityOn() : .connectivityWasOn(
                    continueAction: {
                        resetWarningAction.resetConnectivityWarnings()
                        shouldPresentExportKeysWarningModal.toggle()
                    }()
                ),
                isShowingBottomAlert: $isPresentingConnectivityAlert
            )
            .clearModalBackground()
        }
    }

    func checkForActionsPresentation() {
        if shouldPresentExportKeysWarningModal {
            shouldPresentExportKeysWarningModal.toggle()
            if data.alert {
                isPresentingConnectivityAlert.toggle()
            } else {
                KeyDetailsPublicKeyView.exportPrivateKeyViewModel = exportPrivateKeyService.exportPrivateKey()
                isPresentingExportKeysWarningModal.toggle()
            }
        }
        if shouldPresentRemoveConfirmationModal {
            shouldPresentRemoveConfirmationModal.toggle()
            isShowingRemoveConfirmation.toggle()
        }
    }
}

struct KeyDetailsPublicKeyView_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            KeyDetailsPublicKeyView(
                forgetKeyActionHandler: ForgetSingleKeyAction(navigation: NavigationCoordinator()),
                viewModel: PreviewData.exampleKeyDetailsPublicKey(),
                actionModel: KeyDetailsPublicKeyActionModel(removeSeed: ""),
                exportPrivateKeyService: ExportPrivateKeyService(keyDetails: PreviewData.mkeyDetails),
                resetWarningAction: ResetConnectivtyWarningsAction(alert: Binding<Bool>.constant(false))
            )
            KeyDetailsPublicKeyView(
                forgetKeyActionHandler: ForgetSingleKeyAction(navigation: NavigationCoordinator()),
                viewModel: PreviewData.exampleKeyDetailsPublicKey(isKeyExposed: false),
                actionModel: KeyDetailsPublicKeyActionModel(removeSeed: ""),
                exportPrivateKeyService: ExportPrivateKeyService(keyDetails: PreviewData.mkeyDetails),
                resetWarningAction: ResetConnectivtyWarningsAction(alert: Binding<Bool>.constant(false))
            )
            KeyDetailsPublicKeyView(
                forgetKeyActionHandler: ForgetSingleKeyAction(navigation: NavigationCoordinator()),
                viewModel: PreviewData.exampleKeyDetailsPublicKey(isRootKey: false),
                actionModel: KeyDetailsPublicKeyActionModel(removeSeed: ""),
                exportPrivateKeyService: ExportPrivateKeyService(keyDetails: PreviewData.mkeyDetails),
                resetWarningAction: ResetConnectivtyWarningsAction(alert: Binding<Bool>.constant(false))
            )
            KeyDetailsPublicKeyView(
                forgetKeyActionHandler: ForgetSingleKeyAction(navigation: NavigationCoordinator()),
                viewModel: PreviewData.exampleKeyDetailsPublicKey(isKeyExposed: false, isRootKey: false),
                actionModel: KeyDetailsPublicKeyActionModel(removeSeed: ""),
                exportPrivateKeyService: ExportPrivateKeyService(keyDetails: PreviewData.mkeyDetails),
                resetWarningAction: ResetConnectivtyWarningsAction(alert: Binding<Bool>.constant(false))
            )
        }
        .previewLayout(.sizeThatFits)
        .preferredColorScheme(.dark)
        .environmentObject(NavigationCoordinator())
        .environmentObject(ConnectivityMediator())
    }
}
