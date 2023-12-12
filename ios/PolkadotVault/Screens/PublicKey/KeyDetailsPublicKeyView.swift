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
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        GeometryReader { geo in
            VStack(spacing: 0) {
                // Navigation bar
                NavigationBarView(
                    viewModel: .init(
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
                                    .foregroundColor(.textAndIconsPrimary)
                                Spacer().frame(maxWidth: Spacing.small)
                                Image(.exclamationRed)
                                    .foregroundColor(.accentRed300)
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
                                backgroundColor: .fill6Solid
                            )
                        }
                        .strokeContainerBackground()
                        // Key data
                        keyDetails()
                            .padding(.bottom, Spacing.extraExtraLarge)
                    }
                    .padding(.horizontal, Spacing.large)
                    .padding(.top, Spacing.extraSmall)
                }
            }
            .frame(
                minWidth: geo.size.width,
                minHeight: geo.size.height
            )
            .background(.backgroundPrimary)
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
            isPresented: $viewModel.isPresentingError
        ) {
            ErrorBottomModal(
                viewModel: viewModel.presentableError,
                isShowingBottomAlert: $viewModel.isPresentingError
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
                    .foregroundColor(.textAndIconsTertiary)
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
        .background(
            RoundedRectangle(cornerRadius: CornerRadius.medium)
                .stroke(.fill12, lineWidth: 1)
                .background(.fill6)
                .cornerRadius(CornerRadius.medium)
        )
    }

    @ViewBuilder
    func rowWrapper(
        _ key: String,
        _ value: some View,
        isLast: Bool = false
    ) -> some View {
        HStack(spacing: Spacing.medium) {
            Text(key)
                .foregroundColor(.textAndIconsTertiary)
                .frame(height: Spacing.large, alignment: .center)
            Spacer()
            value
                .frame(idealWidth: .infinity, alignment: .trailing)
                .multilineTextAlignment(.trailing)
                .foregroundColor(.textAndIconsPrimary)
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
    enum OnCompletionAction: Equatable {
        case derivedKeyDeleted
    }

    final class ViewModel: ObservableObject {
        let addressKey: String
        private let publicKeyDetailsService: PublicKeyDetailsServicing
        private let exportPrivateKeyService: ExportPrivateKeyServicing
        private let keyDetailsService: KeyDetailsActionServicing

        @Published var keyDetails: MKeyDetails
        @Published var exportPrivateKeyViewModel: ExportPrivateKeyViewModel!
        @Published var renderable: KeyDetailsPublicKeyViewRenderable
        @Published var isShowingRemoveConfirmation = false
        @Published var isShowingActionSheet = false
        @Published var isPresentingExportKeysWarningModal = false
        @Published var isPresentingExportKeysModal = false
        @Published var shouldPresentExportKeysWarningModal = false
        @Published var shouldPresentExportKeysModal = false
        @Published var shouldPresentRemoveConfirmationModal = false
        @Published var isPresentingError: Bool = false
        @Published var presentableError: ErrorBottomModalViewModel = .alertError(message: "")

        var isExportKeyAvailable: Bool {
            keyDetails.address.hasPwd == false
        }

        var dismissViewRequest: AnyPublisher<Void, Never> {
            dismissRequest.eraseToAnyPublisher()
        }

        private let dismissRequest = PassthroughSubject<Void, Never>()
        private let onCompletion: (OnCompletionAction) -> Void

        init(
            keyDetails: MKeyDetails,
            addressKey: String,
            publicKeyDetailsService: PublicKeyDetailsServicing = PublicKeyDetailsService(),
            exportPrivateKeyService: ExportPrivateKeyServicing = ExportPrivateKeyService(),
            keyDetailsService: KeyDetailsActionServicing = KeyDetailsActionService(),
            onCompletion: @escaping (OnCompletionAction) -> Void
        ) {
            _keyDetails = .init(initialValue: keyDetails)
            self.addressKey = addressKey
            self.publicKeyDetailsService = publicKeyDetailsService
            self.exportPrivateKeyService = exportPrivateKeyService
            self.keyDetailsService = keyDetailsService
            self.onCompletion = onCompletion
            _renderable = .init(initialValue: KeyDetailsPublicKeyViewRenderable(keyDetails))
        }

        func onMoreButtonTap() {
            isShowingActionSheet = true
        }

        func checkForActionsPresentation() {
            if shouldPresentExportKeysWarningModal {
                shouldPresentExportKeysWarningModal = false
                exportPrivateKeyService.exportPrivateKey(keyDetails) { result in
                    switch result {
                    case let .success(model):
                        self.exportPrivateKeyViewModel = model
                        self.isPresentingExportKeysWarningModal = true
                    case let .failure(error):
                        self.presentableError = .alertError(message: error.message)
                        self.isPresentingError = true
                    }
                }
            }
            if shouldPresentRemoveConfirmationModal {
                shouldPresentRemoveConfirmationModal = false
                isShowingRemoveConfirmation = true
            }
        }

        func onWarningDismissal() {
            guard shouldPresentExportKeysModal else { return }
            shouldPresentExportKeysModal = false
            isPresentingExportKeysModal = true
        }

        func onExportKeysDismissal() {
            exportPrivateKeyViewModel = nil
            keyDetailsService.publicKey(
                addressKey: addressKey,
                networkSpecsKey: keyDetails.networkInfo.networkSpecsKey
            ) { result in
                switch result {
                case let .success(keyDetails):
                    self.keyDetails = keyDetails
                    self.renderable = KeyDetailsPublicKeyViewRenderable(keyDetails)
                case let .failure(error):
                    self.presentableError = .alertError(message: error.localizedDescription)
                    self.isPresentingError = true
                }
            }
        }

        func onRemoveKeyTap() {
            publicKeyDetailsService.forgetSingleKey(
                address: addressKey,
                networkSpecsKey: keyDetails.networkInfo.networkSpecsKey
            ) { result in
                switch result {
                case .success:
                    self.onCompletion(.derivedKeyDeleted)
                    self.dismissRequest.send()
                case let .failure(error):
                    self.presentableError = .alertError(message: error.localizedDescription)
                    self.isPresentingError = true
                }
            }
        }
    }
}

#if DEBUG
    struct KeyDetailsPublicKeyView_Previews: PreviewProvider {
        static var previews: some View {
            Group {
                KeyDetailsPublicKeyView(
                    viewModel: .init(
                        keyDetails: .stub,
                        addressKey: "",
                        onCompletion: { _ in }
                    )
                )
            }
            .previewLayout(.sizeThatFits)
            .preferredColorScheme(.dark)
            .environmentObject(ConnectivityMediator())
        }
    }
#endif
