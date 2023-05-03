//
//  EnterBananaSplitPasswordModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 3/1/2023.
//

import SwiftUI

struct EnterBananaSplitPasswordModal: View {
    @EnvironmentObject private var navigation: NavigationCoordinator
    @StateObject var viewModel: ViewModel
    @FocusState var focusedField: SecurePrimaryTextField.Field?
    @FocusState var focusSeedName: Bool
    @State var animateBackground: Bool = false

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
                        .font(PrimaryFont.titleM.font)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .padding(.bottom, Spacing.small)
                    Localizable.EnterBananaSplitPasswordModal.Label.enterName.text
                        .font(PrimaryFont.bodyM.font)
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                        .padding(.bottom, Spacing.extraSmall)
                    TextField("", text: $viewModel.seedName)
                        .primaryTextFieldStyle(
                            Localizable.EnterBananaSplitPasswordModal.Placeholder.enterName.string,
                            text: $viewModel.seedName
                        )
                        .submitLabel(.next)
                        .focused($focusSeedName)
                        .onSubmit {
                            focusedField = .secure
                        }
                        .padding(.bottom, Spacing.small)
                    if !viewModel.isNameValid {
                        Localizable.EnterBananaSplitPasswordModal.Error.Label.invalidSeedName.text
                            .foregroundColor(Asset.accentRed300.swiftUIColor)
                            .font(PrimaryFont.captionM.font)
                            .padding(.bottom, Spacing.small)
                    }
                    Localizable.EnterBananaSplitPasswordModal.Label.enterPassword.text
                        .font(PrimaryFont.bodyM.font)
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                        .padding(.bottom, Spacing.extraSmall)
                    SecurePrimaryTextField(
                        placeholder: Localizable.EnterBananaSplitPasswordModal.Placeholder.enterPassword.string,
                        text: $viewModel.password,
                        isValid: $viewModel.isPasswordValid,
                        focusedField: _focusedField,
                        onCommit: { viewModel.onDoneTap() }
                    )
                    .padding(.bottom, Spacing.small)
                    if !viewModel.isPasswordValid {
                        Localizable.EnterBananaSplitPasswordModal.Error.Label.invalidPassword.text
                            .foregroundColor(Asset.accentRed300.swiftUIColor)
                            .font(PrimaryFont.captionM.font)
                            .padding(.bottom, Spacing.small)
                    }
                }
                .padding(.horizontal, Spacing.large)
                .padding(.bottom, Spacing.small)
            }
            .background(Asset.backgroundTertiary.swiftUIColor)
            .onAppear {
                viewModel.use(navigation: navigation)
                focusSeedName = true
            }
        }
    }
}

extension EnterBananaSplitPasswordModal {
    final class ViewModel: ObservableObject {
        private weak var navigation: NavigationCoordinator!
        @Binding var isPresented: Bool
        @Binding var isKeyRecovered: Bool
        @Binding var isErrorPresented: Bool
        @Binding var presentableError: ErrorBottomModalViewModel
        @Binding var qrCodeData: [String]
        @Binding var onComplete: () -> Void
        @Published var seedName: String = ""
        @Published var password: String = ""
        @Published var isNameValid: Bool = true
        @Published var isPasswordValid: Bool = true
        @Published var isActionDisabled: Bool = true
        @Published var invalidPasswordAttempts: Int = 0
        private var cancelBag = CancelBag()
        private let seedsMediator: SeedsMediating
        private let service: BananaSplitRecoveryService

        init(
            service: BananaSplitRecoveryService = BananaSplitRecoveryService(),
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            isPresented: Binding<Bool>,
            isKeyRecovered: Binding<Bool>,
            isErrorPresented: Binding<Bool>,
            presentableError: Binding<ErrorBottomModalViewModel>,
            qrCodeData: Binding<[String]>,
            onComplete: Binding<() -> Void>
        ) {
            self.service = service
            self.seedsMediator = seedsMediator
            _isPresented = isPresented
            _isKeyRecovered = isKeyRecovered
            _isErrorPresented = isErrorPresented
            _presentableError = presentableError
            _qrCodeData = qrCodeData
            _onComplete = onComplete
            subscribeToUpdates()
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
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
                    dismissWithError(.signingForgotPassword())
                    return
                }
                isPasswordValid = false
            } catch let QrSequenceDecodeError.BananaSplit(s: errorDetail) {
                dismissWithError(.alertError(message: errorDetail))
            } catch let QrSequenceDecodeError.GenericError(s: errorDetail) {
                dismissWithError(.alertError(message: errorDetail))
            } catch {
                dismissWithError(.alertError(message: error.localizedDescription))
            }
        }

        private func performKeySetRecovery(_ seedPhrase: String) {
            if seedsMediator.checkSeedPhraseCollision(seedPhrase: seedPhrase) {
                dismissWithError(.seedPhraseAlreadyExists())
                return
            }
            service.startBananaSplitRecover(seedName, isFirstSeed: seedsMediator.seedNames.isEmpty)
            // We should do additional check on whether seed can be successfully saved and not call navigation
            // further if there are any issues (i.e. somehow seedname is still empty, etc)
            guard seedsMediator.createSeed(
                seedName: seedName,
                seedPhrase: seedPhrase,
                shouldCheckForCollision: false
            ) else {
                dismissWithError(.alertError(
                    message: Localizable.EnterBananaSplitPasswordModal.Error
                        .LocalRestoreFailed.message.string
                ))
                return
            }
            service.completeBananaSplitRecovery(seedPhrase)
            isKeyRecovered = true
            isPresented = false
        }

        private func dismissWithError(_ model: ErrorBottomModalViewModel) {
            presentableError = model
            isErrorPresented = true
            isPresented = false
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
    }
}

#if DEBUG
    struct EnterBananaSplitPasswordModal_Previews: PreviewProvider {
        static var previews: some View {
            EnterBananaSplitPasswordModal(
                viewModel: .init(
                    isPresented: .constant(true),
                    isKeyRecovered: .constant(false),
                    isErrorPresented: .constant(false),
                    presentableError: .constant(.signingForgotPassword()),
                    qrCodeData: .constant([]),
                    onComplete: .constant {}
                )
            )
            .environmentObject(NavigationCoordinator())
            .preferredColorScheme(.dark)
        }
    }
#endif
