//
//  SignSpecEnterPasswordModal.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 27/03/2023.
//

import SwiftUI

struct SignSpecEnterPasswordModal: View {
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
                Text(viewModel.selectedKeyRecord.address.seedName)
                    .foregroundColor(.textAndIconsPrimary)
                    .font(PrimaryFont.bodyM.font)
                HStack {
                    Text(
                        viewModel.selectedKeyRecord.publicKey
                            .truncateMiddle()
                    )
                    .foregroundColor(.textAndIconsTertiary)
                    .font(PrimaryFont.bodyM.font)
                }
            }
            Spacer()
            NetworkIdenticon(
                identicon: viewModel.selectedKeyRecord.address.identicon,
                network: viewModel.selectedKeyRecord.networkLogo,
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
            "\(viewModel.selectedKeyRecord.address.path)\(Localizable.Shared.Label.passwordedPathDelimeter.string)\(Image(.lock))"
        )
    }
}

extension SignSpecEnterPasswordModal {
    final class ViewModel: ObservableObject {
        @Binding var isPresented: Bool

        @Published var password: String = ""
        @Published var isActionDisabled: Bool = true
        @Published var isValid: Bool = true
        let selectedKeyRecord: MRawKey
        private var cancelBag = CancelBag()
        private let onDoneTapAction: (SignSpecEnterPasswordModal.ViewModel) -> Void

        init(
            isPresented: Binding<Bool>,
            selectedKeyRecord: MRawKey,
            onDoneTapAction: @escaping ((SignSpecEnterPasswordModal.ViewModel) -> Void)
        ) {
            _isPresented = isPresented
            self.selectedKeyRecord = selectedKeyRecord
            self.onDoneTapAction = onDoneTapAction
            subscribeToUpdates()
        }

        func onCancelTap() {
            isPresented = false
        }

        func onDoneTap() {
            onDoneTapAction(self)
        }

        private func subscribeToUpdates() {
            $password.sink { newValue in
                self.isActionDisabled = newValue.isEmpty
                self.isValid = true
            }
            .store(in: cancelBag)
        }
    }
}

#if DEBUG
    struct SignSpecEnterPasswordModal_Previews: PreviewProvider {
        static var previews: some View {
            SignSpecEnterPasswordModal(
                viewModel: .init(
                    isPresented: Binding<Bool>.constant(true),
                    selectedKeyRecord: .stub,
                    onDoneTapAction: { _ in }
                )
            )
        }
    }
#endif
