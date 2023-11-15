//
//  EnterBananaSplitPasswordModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 3/1/2023.
//

import SwiftUI

struct EnterBananaSplitPasswordView: View {
    @StateObject var viewModel: ViewModel
    @FocusState var focusedField: SecurePrimaryTextField.Field?
    @FocusState var focusSeedName: Bool
    @State var animateBackground: Bool = false

    var body: some View {
        NavigationView {
            VStack(spacing: Spacing.medium) {
                NavigationBarView(
                    viewModel: .init(
                        title: .progress(current: 1, upTo: 2),
                        leftButtons: [.init(
                            type: .xmark,
                            action: viewModel.onCancelTap
                        )],
                        rightButtons: [.init(
                            type: .activeAction(
                                Localizable.EnterBananaSplitPasswordView.Action.next.key,
                                $viewModel.isActionDisabled
                            ),
                            action: viewModel.onDoneTap
                        )]
                    )
                )
                VStack(alignment: .leading, spacing: 0) {
                    Localizable.EnterBananaSplitPasswordView.Label.title.text
                        .font(PrimaryFont.titleM.font)
                        .foregroundColor(.textAndIconsPrimary)
                        .padding(.bottom, Spacing.small)
                    Localizable.EnterBananaSplitPasswordView.Label.enterName.text
                        .font(PrimaryFont.bodyM.font)
                        .foregroundColor(.textAndIconsSecondary)
                        .padding(.bottom, Spacing.extraSmall)
                    TextField("", text: $viewModel.seedName)
                        .primaryTextFieldStyle(
                            Localizable.EnterBananaSplitPasswordView.Placeholder.enterName.string,
                            text: $viewModel.seedName
                        )
                        .submitLabel(.next)
                        .focused($focusSeedName)
                        .onSubmit {
                            focusedField = .secure
                        }
                        .padding(.bottom, Spacing.small)
                    if !viewModel.isNameValid {
                        Localizable.EnterBananaSplitPasswordView.Error.Label.invalidSeedName.text
                            .foregroundColor(.accentRed300)
                            .font(PrimaryFont.captionM.font)
                            .padding(.bottom, Spacing.small)
                    }
                    Localizable.EnterBananaSplitPasswordView.Label.enterPassword.text
                        .font(PrimaryFont.bodyM.font)
                        .foregroundColor(.textAndIconsSecondary)
                        .padding(.bottom, Spacing.extraSmall)
                    SecurePrimaryTextField(
                        placeholder: Localizable.EnterBananaSplitPasswordView.Placeholder.enterPassword.string,
                        text: $viewModel.password,
                        isValid: $viewModel.isPasswordValid,
                        focusedField: _focusedField,
                        onCommit: { viewModel.onDoneTap() }
                    )
                    .onSubmit {
                        focusedField = nil
                    }
                    .padding(.bottom, Spacing.small)
                    if !viewModel.isPasswordValid {
                        Localizable.EnterBananaSplitPasswordView.Error.Label.invalidPassword.text
                            .foregroundColor(.accentRed300)
                            .font(PrimaryFont.captionM.font)
                            .padding(.bottom, Spacing.small)
                    }
                    Spacer()
                    NavigationLink(
                        destination:
                        CreateKeysForNetworksView(
                            viewModel: viewModel.createDerivedKeys()
                        )
                        .navigationBarHidden(true),
                        isActive: $viewModel.isPresentingDetails
                    ) { EmptyView() }
                }
                .padding(.horizontal, Spacing.large)
                .padding(.bottom, Spacing.small)
                Spacer()
            }
            .navigationViewStyle(.stack)
            .navigationBarHidden(true)
            .background(.backgroundPrimary)
            .onAppear {
                focusSeedName = true
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
}

extension EnterBananaSplitPasswordView {
    final class ViewModel: ObservableObject {
        @Binding var isPresented: Bool
        @Binding var qrCodeData: [String]
        @Published var seedName: String = ""
        @Published var password: String = ""
        @Published var isNameValid: Bool = true
        @Published var isPasswordValid: Bool = true
        @Published var isActionDisabled: Bool = true
        @Published var invalidPasswordAttempts: Int = 0

        @Published var isPresentingDetails: Bool = false

        @Published var isPresentingError: Bool = false
        @Published var presentableError: ErrorBottomModalViewModel!
        private var seedPhrase = ""
        private var cancelBag = CancelBag()
        private let seedsMediator: SeedsMediating
        private let onCompletion: (CreateKeysForNetworksView.OnCompletionAction) -> Void

        init(
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            isPresented: Binding<Bool>,
            qrCodeData: Binding<[String]>,
            onCompletion: @escaping (CreateKeysForNetworksView.OnCompletionAction) -> Void
        ) {
            self.seedsMediator = seedsMediator
            self.onCompletion = onCompletion
            _isPresented = isPresented
            _qrCodeData = qrCodeData
            subscribeToUpdates()
        }

        func onCancelTap() {
            isPresented = false
        }

        func onDoneTap() {
            // If user uses 'return' on password field, we should confirm that `isActionDisable` is false, meaning we
            // have all required data to properly resotre seed
            guard !isActionDisabled else { return }
            do {
                let result = try qrparserTryDecodeQrSequence(data: qrCodeData, password: password, cleaned: false)
                if case let .bBananaSplitRecoveryResult(b: bananaResult) = result,
                   case let .recoveredSeed(s: seedPhrase) = bananaResult {
                    performKeySetRecovery(seedPhrase)
                }
            } catch QrSequenceDecodeError.BananaSplitWrongPassword {
                invalidPasswordAttempts += 1
                if invalidPasswordAttempts > 3 {
                    presentError(.signingForgotPassword())
                    return
                }
                isPasswordValid = false
            } catch let QrSequenceDecodeError.BananaSplit(s: errorDetail) {
                presentError(.alertError(message: errorDetail))
            } catch let QrSequenceDecodeError.GenericError(s: errorDetail) {
                presentError(.alertError(message: errorDetail))
            } catch {
                presentError(.alertError(message: error.localizedDescription))
            }
        }

        private func performKeySetRecovery(_ seedPhrase: String) {
            self.seedPhrase = seedPhrase
            isPresentingDetails = true
        }

        private func presentError(_ model: ErrorBottomModalViewModel) {
            presentableError = model
            isPresentingError = true
        }

        private func subscribeToUpdates() {
            $password.sink { newValue in
                self.isActionDisabled = newValue.isEmpty || self.seedName.isEmpty
            }
            .store(in: cancelBag)
            $seedName.sink { newValue in
                self.isActionDisabled = newValue.isEmpty || self.password.isEmpty
                if !newValue.isEmpty {
                    self.isNameValid = !self.seedsMediator.checkSeedCollision(seedName: newValue)
                } else {
                    self.isNameValid = true
                }
            }
            .store(in: cancelBag)
        }

        func createDerivedKeys() -> CreateKeysForNetworksView.ViewModel {
            .init(
                seedName: seedName,
                seedPhrase: seedPhrase,
                mode: .bananaSplit,
                isPresented: $isPresented,
                onCompletion: onCompletion
            )
        }
    }
}

#if DEBUG
    struct EnterBananaSplitPasswordView_Previews: PreviewProvider {
        static var previews: some View {
            EnterBananaSplitPasswordView(
                viewModel: .init(
                    isPresented: .constant(true),
                    qrCodeData: .constant([]),
                    onCompletion: { _ in }
                )
            )
            .preferredColorScheme(.dark)
        }
    }
#endif
