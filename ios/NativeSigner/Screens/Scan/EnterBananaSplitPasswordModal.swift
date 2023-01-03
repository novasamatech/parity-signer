//
//  EnterBananaSplitPasswordModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 3/1/2023.
//

import SwiftUI

struct EnterBananaSplitPasswordModal: View {
    @EnvironmentObject private var navigation: NavigationCoordinator
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
            safeAreaInsetsMode: .full
        ) {
            VStack(spacing: Spacing.medium) {
                HStack {
                    Button(
                        action: viewModel.onCancelTap
                    ) {
                        Asset.xmarkButtonMedium.swiftUIImage
                            .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
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
                    Localizable.EnterBananaSplitPasswordModal.Label.title.text
                        .font(PrimaryFont.titleL.font)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .padding(.bottom, Spacing.extraSmall)
                    SecurePrimaryTextField(
                        placeholder: Localizable.Transaction.EnterPassword.Label.placeholder.string,
                        text: $viewModel.password,
                        isValid: $viewModel.isValid,
                        focusedField: _focusedField,
                        onCommit: { viewModel.onDoneTap() }
                    )
                    .padding(.top, Spacing.medium)
                    if !viewModel.isValid {
                        Localizable.Transaction.EnterPassword.Label.invalidPassword.text
                            .foregroundColor(Asset.accentRed300.swiftUIColor)
                            .font(PrimaryFont.captionM.font)
                            .padding(.top, Spacing.extraSmall)
                            .padding(.bottom, Spacing.small)
                    }
                }
                .padding(.horizontal, Spacing.large)
                .padding(.bottom, Spacing.large)
            }
            .background(Asset.backgroundTertiary.swiftUIColor)
            .onAppear {
                viewModel.use(navigation: navigation)
                focusedField = .secure
            }
        }
    }
}

extension EnterBananaSplitPasswordModal {
    final class ViewModel: ObservableObject {
        private weak var navigation: NavigationCoordinator!
        @Binding var isPresented: Bool
        @Binding var qrCodeData: [String]
        @Published var password: String = ""
        @Published var isValid: Bool = true
        @Published var isActionDisabled: Bool = true
        @Published var invalidPasswordAttempts: Int = 0
        private var cancelBag = CancelBag()

        init(
            isPresented: Binding<Bool>,
            qrCodeData: Binding<[String]>
        ) {
            _isPresented = isPresented
            _qrCodeData = qrCodeData
            subscribeToUpdates()
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func onCancelTap() {
            // We need to figure out how to get back to proper nav state / other actions, when cancelled
            // Most probably:
            // - dismiss modal
//            isPresented.toggle()
            // - turn on camera again
            // - clear camera state
        }

        func onDoneTap() {
            // Here we'll need to call `qrparserGetPacketsTotal` again with input password
            do {
                let result = try qrparserTryDecodeQrSequence(data: qrCodeData, password: password, cleaned: false)
                if case let .bBananaSplitRecoveryResult(b: bananaResult) = result {
                    switch bananaResult {
                    case .requestPassword:
                        if invalidPasswordAttempts >= 3 {
                            print("Too many invalid password attempts")
                            isPresented.toggle()
                            // dismiss and present error state
                            return
                        }
                        isValid = false
                        invalidPasswordAttempts += 1
                    case let .recoveredSeed(s: seed):
                        // success code path:
                        // - dismiss password modal
                        isPresented.toggle()
                        print("Recovered seed: \(seed)")
                        // - navigate to desired screen
                    }
                }
            } catch {
                // error code path
                // - if password invalid
            }
        }

        private func subscribeToUpdates() {
            $password.sink { newValue in
                self.isActionDisabled = newValue.isEmpty
            }
            .store(in: cancelBag)
        }
    }
}

struct EnterBananaSplitPasswordModal_Previews: PreviewProvider {
    static var previews: some View {
        EnterBananaSplitPasswordModal(
            viewModel: .init(
                isPresented: .constant(true),
                qrCodeData: .constant([])
            )
        )
        .environmentObject(NavigationCoordinator())
        .preferredColorScheme(.dark)
    }
}
