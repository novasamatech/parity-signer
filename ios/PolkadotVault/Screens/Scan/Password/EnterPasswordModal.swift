//
//  EnterPasswordModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 24/11/2022.
//

import SwiftUI

struct EnterPasswordModal: View {
    @StateObject var viewModel: ViewModel
    @FocusState var focusedField: SecurePrimaryTextField.Field?
    @State var animateBackground: Bool = false
    @Environment(\.safeAreaInsets) private var safeAreaInsets

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                viewModel.onCancelTap()
            },
            animateBackground: $animateBackground,
            ignoredEdges: .top
        ) {
            VStack(spacing: Spacing.medium) {
                HStack {
                    Button(
                        action: viewModel.onCancelTap
                    ) {
                        Image(.xmarkButtonMedium)
                            .foregroundColor(.textAndIconsSecondary)
                            .frame(
                                width: Heights.navigationButton,
                                height: Heights.navigationButton,
                                alignment: .center
                            )
                    }
                    .padding(.leading, Spacing.extraExtraSmall)
                    Spacer()
                    CapsuleButton(
                        action: viewModel.onDoneTap,
                        title: Localizable.Transaction.EnterPassword.Action.done.string,
                        isDisabled: $viewModel.isActionDisabled
                    )
                }
                .padding(.top, -Spacing.extraSmall)
                .padding(.horizontal, Spacing.extraSmall)
                VStack(alignment: .leading, spacing: 0) {
                    Localizable.Transaction.EnterPassword.Label.title.text
                        .font(PrimaryFont.titleL.font)
                        .foregroundColor(.textAndIconsPrimary)
                        .padding(.bottom, Spacing.extraSmall)
                    keyComponent()
                        .padding(.top, Spacing.medium)
                    SecurePrimaryTextField(
                        placeholder: Localizable.Transaction.EnterPassword.Label.placeholder.string,
                        text: $viewModel.password,
                        isValid: $viewModel.isValid,
                        focusedField: _focusedField,
                        onCommit: { viewModel.onDoneTap() }
                    )
                    .padding(.top, Spacing.medium)
                    Group {
                        if !viewModel.isValid {
                            Localizable.Transaction.EnterPassword.Label.invalidPassword.text
                                .foregroundColor(.accentRed300)
                        }
                        Localizable.Transaction.EnterPassword.Label.explanation.text
                            .foregroundColor(.textAndIconsTertiary)
                    }
                    .font(PrimaryFont.captionM.font)
                    .padding(.top, Spacing.extraSmall)
                    .padding(.bottom, Spacing.small)
                }
                .padding(.horizontal, Spacing.large)
            }
            .background(.backgroundTertiary)
            .onAppear {
                focusedField = .secure
            }
        }
    }

    @ViewBuilder
    func keyComponent() -> some View {
        HStack {
            VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                renderablePath
                    .foregroundColor(.textAndIconsTertiary)
                    .font(PrimaryFont.captionM.font)
                Text(viewModel.dataModel.authorInfo.address.seedName)
                    .foregroundColor(.textAndIconsPrimary)
                    .font(PrimaryFont.bodyM.font)
                HStack {
                    Text(
                        viewModel.dataModel.authorInfo.base58
                            .truncateMiddle()
                    )
                    .foregroundColor(.textAndIconsTertiary)
                    .font(PrimaryFont.bodyM.font)
                }
            }
            Spacer()
            NetworkIdenticon(
                identicon: viewModel.dataModel.authorInfo.address.identicon,
                network: viewModel.dataModel.networkInfo?.networkLogo,
                background: .accentRed300Overlay,
                size: Heights.identiconInCell
            )
        }
        .padding(Spacing.small)
        .containerBackground(state: .error)
    }

    /// Manual string interpolation for `lock` `SFSymbol`
    private var renderablePath: Text {
        Text(
            // swiftlint:disable:next line_length
            "\(viewModel.dataModel.authorInfo.address.path)\(Localizable.Shared.Label.passwordedPathDelimeter.string)\(Image(.lock))"
        )
    }
}

extension EnterPasswordModal {
    final class ViewModel: ObservableObject {
        private let service: ScanTabService
        @Binding var isPresented: Bool
        @Binding var isErrorPresented: Bool
        @Binding var dataModel: MEnterPassword
        @Binding var signature: MSignatureReady?

        @Published var password: String = ""
        @Published var isActionDisabled: Bool = true
        @Published var isValid: Bool = true
        private var cancelBag = CancelBag()

        init(
            service: ScanTabService = ScanTabService(),
            isPresented: Binding<Bool>,
            isErrorPresented: Binding<Bool>,
            dataModel: Binding<MEnterPassword>,
            signature: Binding<MSignatureReady?>
        ) {
            self.service = service
            _isPresented = isPresented
            _isErrorPresented = isErrorPresented
            _dataModel = dataModel
            _signature = signature
            subscribeToUpdates()
        }

        func onCancelTap() {
            service.resetNavigationState()
            isPresented = false
        }

        func onErrorDismiss() {
            isPresented = false
        }

        func onDoneTap() {
            let actionResult = service.attemptPassword(password)
            // If navigation returned `enterPassword`, it means password is invalid
            if case let .enterPassword(value) = actionResult?.modalData {
                if value.counter > 3 {
                    proceedtoErrorState()
                    return
                }
                dataModel = value
                isValid = false
            }
            // If we got signature from navigation, we should return to camera view and there check for further
            // navigation to Transaction Details
            if case let .signatureReady(value) = actionResult?.modalData {
                isPresented = false
                isErrorPresented = false
                // This needs to trigger navigation to Transaction Details in parent camera view via Binding
                signature = value
                return
            }
            // If we got `Log`, we need to hide password modal, "navigate" to camera view and present
            if case .log = actionResult?.screenData {
                proceedtoErrorState()
            }
        }

        private func proceedtoErrorState() {
            service.resetNavigationState()
            isPresented = false
            isErrorPresented = true
        }

        private func subscribeToUpdates() {
            $password.sink { newValue in
                self.isActionDisabled = newValue.isEmpty
            }
            .store(in: cancelBag)
        }
    }
}

#if DEBUG
    struct EnterPasswordModal_Previews: PreviewProvider {
        static var previews: some View {
            EnterPasswordModal(
                viewModel: .init(
                    isPresented: Binding<Bool>.constant(true),
                    isErrorPresented: Binding<Bool>.constant(false),
                    dataModel: Binding<MEnterPassword>.constant(.stub),
                    signature: Binding<MSignatureReady?>.constant(nil)
                )
            )
        }
    }
#endif
