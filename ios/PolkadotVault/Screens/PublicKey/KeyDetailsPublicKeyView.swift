//
//  KeyDetailsPublicKeyView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 13/09/2022.
//

import SwiftUI

struct KeyDetailsPublicKeyViewModel: Equatable {
    let qrCodes: [[UInt8]]
    let footer: QRCodeAddressFooterViewModel
    let isKeyExposed: Bool
    let isRootKey: Bool

    init(_ keyDetails: MKeyDetails) {
        qrCodes = [keyDetails.qr.payload]
        footer = .init(
            identicon: keyDetails.address.identicon,
            rootKeyName: keyDetails.address.seedName,
            path: keyDetails.address.path,
            hasPassword: keyDetails.address.hasPwd,
            network: keyDetails.networkInfo.networkTitle,
            networkLogo: keyDetails.networkInfo.networkLogo,
            base58: keyDetails.base58
        )
        isKeyExposed = keyDetails.address.secretExposed
        isRootKey = keyDetails.isRootKey
    }

    init(
        qrCodes: [[UInt8]],
        footer: QRCodeAddressFooterViewModel,
        isKeyExposed: Bool,
        isRootKey: Bool
    ) {
        self.qrCodes = qrCodes
        self.footer = footer
        self.isKeyExposed = isKeyExposed
        self.isRootKey = isRootKey
    }
}

struct KeyDetailsPublicKeyView: View {
    @StateObject var viewModel: ViewModel

    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator
    @Environment(\.presentationMode) var mode: Binding<PresentationMode>

    var body: some View {
        GeometryReader { geo in
            VStack(spacing: 0) {
                // Navigation bar
                NavigationBarView(
                    viewModel: .init(
                        title: Localizable.PublicKeyDetails.Label.title.string,
                        subtitle: viewModel.navigationSubtitle(),
                        leftButtons: [.init(type: .arrow, action: {
                            viewModel.onBackTap()
                            mode.wrappedValue.dismiss()
                        })],
                        rightButtons: [.init(type: .more, action: viewModel.onMoreButtonTap)]
                    )
                )
                ScrollView {
                    VStack {
                        VStack(spacing: 0) {
                            AnimatedQRCodeView(
                                viewModel: Binding<AnimatedQRCodeViewModel>.constant(
                                    .init(
                                        qrCodes: viewModel.renderable.qrCodes
                                    )
                                )
                            )
                            .padding(Spacing.stroke)
                            QRCodeAddressFooterView(
                                viewModel: viewModel.renderable.footer,
                                backgroundColor: Asset.fill6Solid.swiftUIColor
                            )
                        }
                        .strokeContainerBackground()
                        // Exposed key alert
                        if viewModel.renderable.isKeyExposed {
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
                    .padding([.top, .bottom], Spacing.flexibleComponentSpacer)
                    Spacer()
                }
            }
            .frame(
                minWidth: geo.size.width,
                minHeight: geo.size.height
            )
            .background(Asset.backgroundPrimary.swiftUIColor)
            .onAppear {
                viewModel.use(navigation: navigation)
                viewModel.onAppear()
            }
        }
        // Action sheet
        .fullScreenCover(
            isPresented: $viewModel.isShowingActionSheet,
            onDismiss: {
                // iOS 15 handling of following .fullscreen presentation after dismissal, we need to dispatch this async
                DispatchQueue.main.async { viewModel.checkForActionsPresentation() }
            }
        ) {
            PublicKeyActionsModal(
                shouldPresentExportKeysWarningModal: $viewModel.shouldPresentExportKeysWarningModal,
                isShowingActionSheet: $viewModel.isShowingActionSheet,
                shouldPresentRemoveConfirmationModal: $viewModel.shouldPresentRemoveConfirmationModal
            )
            .clearModalBackground()
        }
        // Export private key warning
        .fullScreenCover(
            isPresented: $viewModel.isPresentingExportKeysWarningModal,
            onDismiss: {
                // iOS 15 handling of following .fullscreen presentation after dismissal, we need to dispatch this async
                DispatchQueue.main.async { viewModel.onWarningDismissal() }
            }
        ) {
            ExportPrivateKeyWarningModal(
                isPresentingExportKeysWarningModal: $viewModel.isPresentingExportKeysWarningModal,
                shouldPresentExportKeysModal: $viewModel.shouldPresentExportKeysModal
            )
            .clearModalBackground()
        }
        // Export private key modal
        .fullScreenCover(
            isPresented: $viewModel.isPresentingExportKeysModal,
            onDismiss: viewModel.onExportKeysDismissal
        ) {
            ExportPrivateKeyModal(
                isPresentingExportKeysModal: $viewModel.isPresentingExportKeysModal,
                viewModel: viewModel.exportPrivateKeyViewModel
            )
            .clearModalBackground()
        }
        // Remove key modal
        .fullScreenCover(isPresented: $viewModel.isShowingRemoveConfirmation) {
            HorizontalActionsBottomModal(
                viewModel: .forgetSingleKey,
                mainAction: viewModel.onRemoveKeyTap(),
                dismissAction: viewModel.onRemoveKeyDismissal(),
                isShowingBottomAlert: $viewModel.isShowingRemoveConfirmation
            )
            .clearModalBackground()
        }
        .fullScreenCover(
            isPresented: $viewModel.isPresentingConnectivityAlert,
            onDismiss: {
                // iOS 15 handling of following .fullscreen presentation after dismissal, we need to dispatch this async
                DispatchQueue.main.async { viewModel.checkForActionsPresentation() }
            }
        ) {
            ErrorBottomModal(
                viewModel: connectivityMediator.isConnectivityOn ? .connectivityOn() : .connectivityWasOn(
                    continueAction: viewModel.onConnectivityErrorContinueTap()
                ),
                isShowingBottomAlert: $viewModel.isPresentingConnectivityAlert
            )
            .clearModalBackground()
        }
    }
}

extension KeyDetailsPublicKeyView {
    final class ViewModel: ObservableObject {
        private let keyDetails: MKeyDetails
        private let forgetKeyActionHandler: ForgetSingleKeyAction
        private let exportPrivateKeyService: ExportPrivateKeyService
        private let warningStateMediator: WarningStateMediator

        private weak var navigation: NavigationCoordinator!
        @Published var exportPrivateKeyViewModel: ExportPrivateKeyViewModel!
        @Published var renderable: KeyDetailsPublicKeyViewModel
        @Published var isShowingRemoveConfirmation = false
        @Published var isShowingActionSheet = false
        @Published var isPresentingExportKeysWarningModal = false
        @Published var isPresentingExportKeysModal = false
        @Published var isPresentingConnectivityAlert = false
        @Published var shouldPresentExportKeysWarningModal = false
        @Published var shouldPresentExportKeysModal = false
        @Published var shouldPresentRemoveConfirmationModal = false

        init(
            keyDetails: MKeyDetails,
            forgetKeyActionHandler: ForgetSingleKeyAction = ForgetSingleKeyAction(),
            exportPrivateKeyService: ExportPrivateKeyService = ExportPrivateKeyService(),
            warningStateMediator: WarningStateMediator = ServiceLocator.warningStateMediator
        ) {
            self.keyDetails = keyDetails
            self.forgetKeyActionHandler = forgetKeyActionHandler
            self.exportPrivateKeyService = exportPrivateKeyService
            self.warningStateMediator = warningStateMediator
            _renderable = .init(initialValue: KeyDetailsPublicKeyViewModel(keyDetails))
        }

        func onAppear() {
            navigation.performFake(navigation: .init(action: .rightButtonAction))
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
            forgetKeyActionHandler.use(navigation: navigation)
        }

        func onBackTap() {
            navigation.performFake(navigation: .init(action: .goBack))
        }

        func onMoreButtonTap() {
            isShowingActionSheet.toggle()
        }

        func navigationSubtitle() -> String? {
            renderable.isRootKey ? nil : Localizable.PublicKeyDetails.Label.subtitle.string
        }

        func checkForActionsPresentation() {
            if shouldPresentExportKeysWarningModal {
                shouldPresentExportKeysWarningModal.toggle()
                if warningStateMediator.alert {
                    isPresentingConnectivityAlert.toggle()
                } else {
                    exportPrivateKeyViewModel = exportPrivateKeyService.exportPrivateKey(keyDetails)
                    isPresentingExportKeysWarningModal.toggle()
                }
            }
            if shouldPresentRemoveConfirmationModal {
                shouldPresentRemoveConfirmationModal.toggle()
                isShowingRemoveConfirmation.toggle()
            }
        }

        func onWarningDismissal() {
            if shouldPresentExportKeysModal {
                shouldPresentExportKeysModal.toggle()
                isPresentingExportKeysModal.toggle()
            } else {
                // If user cancelled, mimic Rust state machine and hide "..." modal menu
                navigation.perform(navigation: .init(action: .rightButtonAction))
            }
        }

        func onExportKeysDismissal() {
            // When user finished Export Private Key interaction, mimic Rust state machine and hide "..." modal menu
            navigation.perform(navigation: .init(action: .rightButtonAction))
            exportPrivateKeyViewModel = nil
        }

        func onConnectivityErrorContinueTap() {
            warningStateMediator.resetConnectivityWarnings()
            shouldPresentExportKeysWarningModal.toggle()
        }

        func onRemoveKeyTap() {
            forgetKeyActionHandler.forgetSingleKey(keyDetails.address.seedName)
        }

        func onRemoveKeyDismissal() {
            // We need to fake right button action here or Rust machine will break
            // In old UI, if you dismiss equivalent of this modal, underlying modal would still be there,
            // so we need to inform Rust we actually hid it
            navigation.performFake(navigation: .init(action: .rightButtonAction))
        }
    }

    struct KeyDetailsPublicKeyView_Previews: PreviewProvider {
        static var previews: some View {
            Group {
                KeyDetailsPublicKeyView(
                    viewModel: .init(
                        keyDetails: PreviewData.mkeyDetails
                    )
                )
            }
            .previewLayout(.sizeThatFits)
            .preferredColorScheme(.dark)
            .environmentObject(NavigationCoordinator())
            .environmentObject(ConnectivityMediator())
        }
    }
}
