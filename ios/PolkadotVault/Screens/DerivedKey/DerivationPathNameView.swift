//
//  DerivationPathNameView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 11/01/2023.
//

import Combine
import SwiftUI

struct DerivationPathNameView: View {
    @FocusState var focusedPath: Bool
    @FocusState var focusedField: SecurePrimaryTextField.Field?
    @StateObject var viewModel: ViewModel
    @State var isUpdatingText = false
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: .title(Localizable.CreateDerivedKey.Modal.Path.title.string),
                    leftButtons: [.init(
                        type: .arrow,
                        action: { presentationMode.wrappedValue.dismiss() }
                    )],
                    rightButtons: [.init(
                        type: .activeAction(
                            Localizable.CreateDerivedKey.Modal.Path.Action.navigation.key,
                            $viewModel.isMainActionDisabled
                        ),
                        action: viewModel.onRightNavigationButtonTap
                    )],
                    backgroundColor: .backgroundPrimary
                )
            )
            .padding(.bottom, Spacing.extraSmall)
            // Content
            Localizable.CreateDerivedKey.Modal.Path.header.text
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.bodyL.font)
                .padding(.horizontal, Spacing.large)
                .padding(.bottom, Spacing.small)
            ScrollView(showsIndicators: false) {
                VStack(alignment: .leading, spacing: 0) {
                    TextField("", text: $viewModel.inputText)
                        .primaryTextFieldStyle(
                            Localizable.CreateDerivedKey.Modal.Path.Placeholder.path.string,
                            text: $viewModel.inputText,
                            isValid: .constant(viewModel.derivationPathError == nil)
                        )
                        .autocorrectionDisabled()
                        .submitLabel(.next)
                        .focused($focusedPath)
                        .onSubmit {
                            if viewModel.isPassworded {
                                focusedField = .secure
                            }
                        }
                        .onChange(of: viewModel.inputText) { newValue in
                            viewModel.isPassworded = newValue
                                .contains(DerivationPathComponent.passworded.description)
                            viewModel.validateDerivationPath()
                        }
                        .padding(.bottom, Spacing.extraSmall)
                    if let derivationPathError = viewModel.derivationPathError {
                        Text(derivationPathError)
                            .foregroundColor(.accentRed300)
                            .font(PrimaryFont.captionM.font)
                            .padding(.bottom, Spacing.small)
                    }
                    if viewModel.isEntrySuggestionActive {
                        Localizable.CreateDerivedKey.Modal.Path.Suggestion.path.text
                            .foregroundColor(.textAndIconsPrimary)
                            .font(PrimaryFont.captionM.font)
                            .padding(.bottom, Spacing.small)
                    }
                    quickActions()
                        .padding(.bottom, Spacing.extraSmall)
                    Localizable.CreateDerivedKey.Modal.Path.Footer.path.text
                        .foregroundColor(.textAndIconsTertiary)
                        .font(PrimaryFont.captionM.font)
                        .padding(.vertical, Spacing.extraSmall)
                    if viewModel.isPassworded {
                        Localizable.CreateDerivedKey.Modal.Path.Header.password.text
                            .font(PrimaryFont.bodyL.font)
                            .foregroundColor(.textAndIconsPrimary)
                            .padding(.bottom, Spacing.medium)
                            .padding(.top, Spacing.medium)
                        SecurePrimaryTextField(
                            placeholder: Localizable.CreateDerivedKey.Modal.Path.Placeholder.password.string,
                            text: $viewModel.passwordConfirmation,
                            isValid: $viewModel.isPasswordValid,
                            focusedField: _focusedField,
                            onCommit: {
                                focusedField = nil
                                focusedPath = false
                                viewModel.onPasswordConfirmationDoneTap()
                            }
                        )
                        .padding(.bottom, Spacing.small)
                        if !viewModel.isPasswordValid {
                            Localizable.CreateDerivedKey.Modal.Path.Error.password.text
                                .foregroundColor(.accentRed300)
                                .font(PrimaryFont.captionM.font)
                                .padding(.bottom, Spacing.small)
                        }
                    }
                    AttributedInfoBoxView(text: Localizable.createDerivedKeyModalPathInfo())
                        .onTapGesture { viewModel.onInfoBoxTap() }
                        .padding(.vertical, Spacing.extraSmall)
                    Spacer()
                }
                .padding(.horizontal, Spacing.large)
                .padding(.bottom, Spacing.medium)
            }
            .background(.backgroundPrimary)
        }
        .background(.backgroundPrimary)
        .onAppear {
            focusedPath = true
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingInfoModal
        ) {
            DerivationMethodsInfoView(
                viewModel: .init(
                    isPresented: $viewModel.isPresentingInfoModal
                )
            )
            .clearModalBackground()
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingConfirmation
        ) {
            CreateDerivedKeyConfirmationView(
                viewModel: .init(
                    derivationPath: viewModel.unwrappedDerivationPath(),
                    onCompletion: viewModel.onConfirmationCompletion
                )
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

    @ViewBuilder
    func quickActions() -> some View {
        HStack(spacing: Spacing.extraExtraSmall) {
            Localizable.CreateDerivedKey.Modal.Path.Action.softPath.text
                .asSoftCapsuleButton()
                .onTapGesture {
                    viewModel.onSoftPathTap()
                }
            Localizable.CreateDerivedKey.Modal.Path.Action.hardPath.text
                .asSoftCapsuleButton()
                .onTapGesture {
                    viewModel.onHardPathTap()
                }
            Localizable.CreateDerivedKey.Modal.Path.Action.passwordedPath.text
                .asSoftCapsuleButton()
                .onTapGesture {
                    viewModel.onPasswordedPathTap()
                }
            Spacer()
        }
    }
}

struct SoftCapsuleButton: ViewModifier {
    func body(content: Content) -> some View {
        content
            .foregroundColor(.accentPink300)
            .font(PrimaryFont.labelS.font)
            .padding(.vertical, Spacing.extraSmall)
            .padding(.horizontal, Spacing.medium)
            .background(.accentPink12)
            .clipShape(Capsule())
    }
}

extension View {
    func asSoftCapsuleButton() -> some View {
        modifier(SoftCapsuleButton())
    }
}

extension DerivationPathNameView {
    final class ViewModel: ObservableObject {
        private let createKeyService: CreateDerivedKeyService
        private let createKeyNameService: CreateDerivedKeyNameService
        private let seedName: String
        private let keySet: MKeysNew
        private let networkSelection: MmNetwork
        private let onComplete: () -> Void
        private var skipValidation = false
        private let cancelBag = CancelBag()

        @Published var inputText: String = ""
        @Published var maskedText: String = ""
        @Published var passwordConfirmation: String = ""
        @Published var isPassworded: Bool = false
        @Published var isPasswordValid: Bool = true
        @Published var isMainActionDisabled: Bool = true
        @Published var derivationPathError: String?
        @Published var isPresentingConfirmation: Bool = false
        @Published var derivationPath: String?

        var isEntrySuggestionActive: Bool {
            DerivationPathComponent.allCases.contains { inputText == $0.description } && derivationPathError == nil
        }

        // State presentatation
        @Published var isPresentingInfoModal: Bool = false
        @Published var isPresentingError: Bool = false
        @Published var presentableError: ErrorBottomModalViewModel!

        init(
            seedName: String,
            keySet: MKeysNew,
            networkSelection: MmNetwork,
            createKeyService: CreateDerivedKeyService = CreateDerivedKeyService(),
            createKeyNameService: CreateDerivedKeyNameService = CreateDerivedKeyNameService(),
            onComplete: @escaping () -> Void
        ) {
            self.seedName = seedName
            self.keySet = keySet
            self.networkSelection = networkSelection
            self.createKeyService = createKeyService
            self.createKeyNameService = createKeyNameService
            self.onComplete = onComplete
            subscribeToChanges()
            prefillTextField()
        }

        func onRightNavigationButtonTap() {
            derivationPath = inputText
            let completion: (Result<Void, ServiceError>) -> Void = { result in
                switch result {
                case .success:
                    self.isPresentingConfirmation = true
                case let .failure(error):
                    self.presentableError = .alertError(message: error.localizedDescription)
                    self.isPresentingError = true
                }
            }
            createKeyService.createDerivedKey(
                seedName,
                unwrappedDerivationPath(),
                networkSelection.key,
                completion
            )
        }

        func onInfoBoxTap() {
            isPresentingInfoModal = true
        }

        func onSoftPathTap() {
            inputText.append(DerivationPathComponent.soft.description)
        }

        func onHardPathTap() {
            inputText.append(DerivationPathComponent.hard.description)
        }

        func onPasswordedPathTap() {
            inputText.append(DerivationPathComponent.passworded.description)
        }

        func onConfirmationCompletion() {
            isPresentingConfirmation = false
            onComplete()
        }

        func validateDerivationPath() {
            guard !skipValidation else {
                skipValidation = false
                return
            }
            createKeyService.checkForCollision(
                seedName,
                inputText,
                networkSelection.key,
                completion: pathErrorCheck(_:)
            )
        }

        func onPasswordConfirmationDoneTap() {
            isPasswordValid = isPasswordConfirmationValid()
        }

        func isPasswordConfirmationValid() -> Bool {
            guard isPassworded else { return true }
            if let range = inputText.range(of: DerivationPathComponent.passworded.description) {
                let substring = inputText.suffix(from: range.upperBound)
                return substring == passwordConfirmation
            }
            return false
        }

        func unwrappedDerivationPath() -> String {
            derivationPath ?? ""
        }
    }
}

private extension DerivationPathNameView.ViewModel {
    func prefillTextField() {
        if derivationPath == nil {
            inputText = createKeyNameService.defaultDerivedKeyName(keySet, network: networkSelection)
            validateDerivationPath()
        } else {
            inputText = derivationPath ?? ""
        }
    }

    func subscribeToChanges() {
        Publishers
            .CombineLatest3($isPassworded, $isPasswordValid, $derivationPathError)
            .map { validators -> Bool in
                let (isPassworded, isPasswordValid, derivationPathError) = validators
                if isPassworded {
                    return
                        !isPasswordValid ||
                        !self.isPasswordConfirmationValid() ||
                        self.derivationPathError != nil

                } else {
                    return derivationPathError != nil || self.isInitialEntry()
                }
            }
            .assign(to: \.isMainActionDisabled, on: self)
            .store(in: cancelBag)
    }

    func isInitialEntry() -> Bool {
        inputText == DerivationPathComponent.hard.description
    }

    func pathErrorCheck(_ result: Result<DerivationCheck, ServiceError>) {
        switch result {
        case let .success(derivationCheck):
            if derivationCheck.collision != nil {
                derivationPathError = Localizable.CreateDerivedKey.Modal.Path.Error.alreadyExists.string
            } else if !derivationCheck.buttonGood {
                derivationPathError = Localizable.CreateDerivedKey.Modal.Path.Error.pathInvalid.string
            } else if derivationCheck.error != nil {
                derivationPathError = derivationCheck.error
            } else {
                derivationPathError = nil
            }
        case let .failure(error):
            presentableError = .alertError(message: error.backendDisplayError)
            isPresentingError = true
        }
    }
}

#if DEBUG
    struct DerivationPathNameView_Previews: PreviewProvider {
        static var previews: some View {
            DerivationPathNameView(
                viewModel: .init(
                    seedName: "Keys",
                    keySet: .stub,
                    networkSelection: .stub,
                    onComplete: {}
                )
            )
        }
    }
#endif
