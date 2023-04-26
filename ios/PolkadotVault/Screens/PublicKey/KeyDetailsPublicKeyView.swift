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
    let networkTitle: String
    let networkLogo: String
    let keySetName: String
    let path: String
    let hasPassword: Bool

    init(_ keyDetails: MKeyDetails) {
        qrCodes = [keyDetails.qr.payload]
        footer = .init(
            identicon: keyDetails.address.identicon,

            networkLogo: keyDetails.networkInfo.networkLogo,
            base58: keyDetails.base58
        )
        isKeyExposed = keyDetails.address.secretExposed
        isRootKey = keyDetails.isRootKey
        networkTitle = keyDetails.networkInfo.networkTitle
        networkLogo = keyDetails.networkInfo.networkLogo
        keySetName = keyDetails.address.seedName
        path = keyDetails.address.path
        hasPassword = keyDetails.address.hasPwd
    }

    init(
        qrCodes: [[UInt8]],
        footer: QRCodeAddressFooterViewModel,
        isKeyExposed: Bool,
        isRootKey: Bool,
        networkTitle: String,
        networkLogo: String,
        keySetName: String,
        path: String,
        hasPassword: Bool
    ) {
        self.qrCodes = qrCodes
        self.footer = footer
        self.isKeyExposed = isKeyExposed
        self.isRootKey = isRootKey
        self.networkTitle = networkTitle
        self.networkLogo = networkLogo
        self.keySetName = keySetName
        self.path = path
        self.hasPassword = hasPassword
    }
}

struct KeyDetailsPublicKeyView: View {
    @StateObject var viewModel: ViewModel

    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator

    var body: some View {
        GeometryReader { geo in
            VStack(spacing: 0) {
                // Navigation bar
                NavigationBarView(
                    viewModel: .init(
                        title: Localizable.PublicKeyDetails.Label.title.string,
                        subtitle: viewModel.navigationSubtitle(),
                        leftButtons: [.init(
                            type: .xmark,
                            action: viewModel.onBackTap
                        )],
                        rightButtons: [.init(type: .more, action: viewModel.onMoreButtonTap)]
                    )
                )
                ScrollView {
                    VStack(spacing: Spacing.medium) {
                        // Exposed key alert
                        if viewModel.renderable.isKeyExposed {
                            HStack(alignment: .center, spacing: 0) {
                                Localizable.KeyScreen.Label.hotkey.text
                                    .frame(maxWidth: .infinity, alignment: .leading)
                                    .font(PrimaryFont.labelXS.font)
                                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                                Spacer().frame(maxWidth: Spacing.small)
                                Asset.exclamationRed.swiftUIImage
                                    .foregroundColor(Asset.accentRed300.swiftUIColor)
                            }
                            .padding(Spacing.medium)
                            .strokeContainerBackground(CornerRadius.small, state: .error)
                        }
                        // QR Code container
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
                        // Key data
                        keyDetails()
                            .padding(.bottom, Spacing.extraExtraLarge)
                    }
                    .padding([.leading, .trailing], Spacing.large)
                    .padding(.top, Spacing.extraSmall)
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

private extension KeyDetailsPublicKeyView {
    @ViewBuilder
    func keyDetails() -> some View {
        VStack(alignment: .leading, spacing: 0) {
            HStack(spacing: 0) {
                Localizable.PublicKeyDetails.Label.network.text
                    .frame(height: Spacing.large, alignment: .center)
                    .padding(.vertical, Spacing.small)
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                Spacer()
                NetworkIconCapsuleView(
                    networkLogo: viewModel.renderable.networkLogo,
                    networkTitle: viewModel.renderable.networkTitle
                )
            }
            Divider()
            rowWrapper(
                Localizable.PublicKeyDetails.Label.derivation.string,
                viewModel.renderable.path.isEmpty && !viewModel.renderable.hasPassword ? Localizable.PublicKeyDetails
                    .Label.emptyPath.text : fullPath
            )
            rowWrapper(
                Localizable.PublicKeyDetails.Label.keySetName.string,
                Text(viewModel.renderable.keySetName),
                isLast: true
            )
        }
        .font(PrimaryFont.bodyL.font)
        .padding(.horizontal, Spacing.medium)
        .containerBackground()
    }

    @ViewBuilder
    func rowWrapper(
        _ key: String,
        _ value: some View,
        isLast: Bool = false
    ) -> some View {
        HStack(spacing: Spacing.medium) {
            Text(key)
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                .frame(height: Spacing.large, alignment: .center)
            Spacer()
            value
                .frame(idealWidth: .infinity, alignment: .trailing)
                .multilineTextAlignment(.trailing)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
        }
        .padding(.vertical, Spacing.small)
        if !isLast {
            Divider()
        }
    }

    /// String interpolation for SFSymbols is a bit unstable if creating `String` inline by using conditional logic or
    /// `appending` from `StringProtocol`. Hence less DRY approach and dedicated function to wrap that
    var fullPath: Text {
        viewModel.renderable.hasPassword ?
            Text(
                "\(viewModel.renderable.path)\(Localizable.Shared.Label.passwordedPathDelimeter.string)\(Image(.lock))"
            ) :
            Text(viewModel.renderable.path)
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
            navigation.perform(navigation: .init(action: .goBack))
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
