//
//  KeyDetailsPublicKeyView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 13/09/2022.
//

import Combine
import SwiftUI

struct KeyDetailsPublicKeyView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        GeometryReader { geo in
            VStack(spacing: 0) {
                // Navigation bar
                NavigationBarView(
                    viewModel: .init(
                        title: Localizable.PublicKeyDetails.Label.title.string,
                        subtitle: viewModel.navigationSubtitle(),
                        leftButtons: [.init(type: .arrow, action: { presentationMode.wrappedValue.dismiss() })],
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
        }
        .onReceive(viewModel.dismissViewRequest) { _ in
            presentationMode.wrappedValue.dismiss()
        }
        // Action sheet
        .fullScreenModal(
            isPresented: $viewModel.isShowingActionSheet,
            onDismiss: {
                // iOS 15 handling of following .fullscreen presentation after dismissal, we need to dispatch this async
                DispatchQueue.main.async { viewModel.checkForActionsPresentation() }
            }
        ) {
            PublicKeyActionsModal(
                shouldPresentExportKeysWarningModal: $viewModel.shouldPresentExportKeysWarningModal,
                isShowingActionSheet: $viewModel.isShowingActionSheet,
                shouldPresentRemoveConfirmationModal: $viewModel.shouldPresentRemoveConfirmationModal,
                isExportKeyAvailable: viewModel.isExportKeyAvailable
            )
            .clearModalBackground()
        }
        // Export private key warning
        .fullScreenModal(
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
        .fullScreenModal(
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
        .fullScreenModal(isPresented: $viewModel.isShowingRemoveConfirmation) {
            HorizontalActionsBottomModal(
                viewModel: .forgetSingleKey,
                mainAction: viewModel.onRemoveKeyTap(),
                isShowingBottomAlert: $viewModel.isShowingRemoveConfirmation
            )
            .clearModalBackground()
        }
        .fullScreenModal(
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
        private let publicKeyDetails: String
        private let publicKeyDetailsService: PublicKeyDetailsService
        private let exportPrivateKeyService: ExportPrivateKeyService
        private let warningStateMediator: WarningStateMediator
        private let snackbarPresentation: BottomSnackbarPresentation

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
        var isExportKeyAvailable: Bool {
            keyDetails.address.hasPwd == false
        }

        var dismissViewRequest: AnyPublisher<Void, Never> {
            dismissRequest.eraseToAnyPublisher()
        }

        private let dismissRequest = PassthroughSubject<Void, Never>()
        private let onCompletion: () -> Void

        init(
            keyDetails: MKeyDetails,
            publicKeyDetails: String,
            publicKeyDetailsService: PublicKeyDetailsService = PublicKeyDetailsService(),
            exportPrivateKeyService: ExportPrivateKeyService = ExportPrivateKeyService(),
            warningStateMediator: WarningStateMediator = ServiceLocator.warningStateMediator,
            snackbarPresentation: BottomSnackbarPresentation = ServiceLocator.bottomSnackbarPresentation,
            onCompletion: @escaping () -> Void
        ) {
            self.keyDetails = keyDetails
            self.publicKeyDetails = publicKeyDetails
            self.publicKeyDetailsService = publicKeyDetailsService
            self.exportPrivateKeyService = exportPrivateKeyService
            self.warningStateMediator = warningStateMediator
            self.snackbarPresentation = snackbarPresentation
            self.onCompletion = onCompletion
            _renderable = .init(initialValue: KeyDetailsPublicKeyViewModel(keyDetails))
        }

        func onMoreButtonTap() {
            isShowingActionSheet.toggle()
        }

        func navigationSubtitle() -> String? {
            renderable.isRootKey ? nil : Localizable.PublicKeyDetails.Label.subtitle.string
        }

        func checkForActionsPresentation() {
            if shouldPresentExportKeysWarningModal {
                shouldPresentExportKeysWarningModal = false
                if warningStateMediator.alert {
                    isPresentingConnectivityAlert = true
                } else {
                    exportPrivateKeyViewModel = exportPrivateKeyService.exportPrivateKey(keyDetails)
                    isPresentingExportKeysWarningModal = true
                }
            }
            if shouldPresentRemoveConfirmationModal {
                shouldPresentRemoveConfirmationModal.toggle()
                isShowingRemoveConfirmation.toggle()
            }
        }

        func onWarningDismissal() {
            guard shouldPresentExportKeysModal else { return }
            shouldPresentExportKeysModal.toggle()
            isPresentingExportKeysModal.toggle()
        }

        func onExportKeysDismissal() {
            exportPrivateKeyViewModel = nil
        }

        func onConnectivityErrorContinueTap() {
            warningStateMediator.resetConnectivityWarnings()
            shouldPresentExportKeysWarningModal.toggle()
        }

        func onRemoveKeyTap() {
            publicKeyDetailsService.forgetSingleKey(keyDetails.address.seedName)
            snackbarPresentation.viewModel = .init(
                title: Localizable.PublicKeyDetailsModal.Confirmation.snackbar.string,
                style: .warning
            )
            snackbarPresentation.isSnackbarPresented = true
            onCompletion()
            dismissRequest.send()
        }
    }
}

struct KeyDetailsPublicKeyView_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            KeyDetailsPublicKeyView(
                viewModel: .init(
                    keyDetails: PreviewData.mkeyDetails,
                    publicKeyDetails: "publicKeyDetails",
                    onCompletion: {}
                )
            )
        }
        .previewLayout(.sizeThatFits)
        .preferredColorScheme(.dark)
        .environmentObject(ConnectivityMediator())
    }
}
