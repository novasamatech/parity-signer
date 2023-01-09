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
        @Published var seedName: String = ""
        @Published var password: String = ""
        @Published var isNameValid: Bool = true
        @Published var isPasswordValid: Bool = true
        @Published var isActionDisabled: Bool = true
        @Published var invalidPasswordAttempts: Int = 0
        private var cancelBag = CancelBag()
        private let seedsMediator: SeedsMediating

        init(
            isPresented: Binding<Bool>,
            isKeyRecovered: Binding<Bool>,
            isErrorPresented: Binding<Bool>,
            presentableError: Binding<ErrorBottomModalViewModel>,
            qrCodeData: Binding<[String]>,
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
        ) {
            _isPresented = isPresented
            _isKeyRecovered = isKeyRecovered
            _isErrorPresented = isErrorPresented
            _presentableError = presentableError
            _qrCodeData = qrCodeData
            self.seedsMediator = seedsMediator
            subscribeToUpdates()
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func onCancelTap() {
            isPresented.toggle()
        }

        func onDoneTap() {
            do {
                let result = try qrparserTryDecodeQrSequence(data: qrCodeData, password: password, cleaned: false)
                if case let .bBananaSplitRecoveryResult(b: bananaResult) = result,
                   case let .recoveredSeed(s: seedPhrase) = bananaResult {
                    if seedsMediator.checkSeedPhraseCollision(seedPhrase: seedPhrase) {
                        dismissWithError(.seedPhraseAlreadyExists())
                        return
                    }
                    navigation.performFake(navigation: .init(action: .navbarKeys))
                    navigation.performFake(navigation: .init(action: .rightButtonAction))
                    navigation.performFake(navigation: .init(action: .recoverSeed))
                    navigation.performFake(navigation: .init(action: .goForward, details: seedName))
                    seedsMediator.restoreSeed(seedName: seedName, seedPhrase: seedPhrase, navigate: false)
                    navigation.performFake(navigation: .init(action: .goBack))
                    navigation.overrideQRScannerDismissalNavigation = .init(action: .selectSeed, details: seedName)
                    isKeyRecovered = true
                    isPresented.toggle()
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

        private func dismissWithError(_ model: ErrorBottomModalViewModel) {
            presentableError = model
            isErrorPresented = true
            isPresented.toggle()
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
                    qrCodeData: .constant([])
                )
            )
            .environmentObject(NavigationCoordinator())
            .preferredColorScheme(.dark)
        }
    }
#endif
