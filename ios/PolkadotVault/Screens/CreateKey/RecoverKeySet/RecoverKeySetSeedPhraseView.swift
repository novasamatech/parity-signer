//
//  RecoverKeySetSeedPhraseView.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 06/03/2023.
//

import SwiftUI

struct RecoverKeySetSeedPhraseView: View {
    @StateObject var viewModel: ViewModel
    @Environment(\.presentationMode) var mode: Binding<PresentationMode>
    @FocusState private var focus: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            NavigationBarView(
                viewModel: .init(
                    title: .progress(current: 2, upTo: 3),
                    leftButtons: [.init(type: .arrow, action: { mode.wrappedValue.dismiss() })],
                    rightButtons: [.init(
                        type: .activeAction(
                            Localizable.RecoverSeedPhrase.Action.next.key,
                            .constant(!viewModel.isValidSeedPhrase)
                        ),
                        action: viewModel.onDoneTap
                    )]
                )
            )
            ScrollView(showsIndicators: false) {
                VStack(alignment: .leading, spacing: 0) {
                    Localizable.RecoverSeedPhrase.Label.title.text
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.titleL.font)
                        .padding(.top, Spacing.extraSmall)
                    Localizable.RecoverSeedPhrase.Label.header.text
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.bodyL.font)
                        .padding(.vertical, Spacing.extraSmall)
                    HStack {
                        Spacer()
                    }
                }
                .padding(.top, Spacing.extraExtraSmall)
                .padding(.bottom, Spacing.medium)
                .padding(.horizontal, Spacing.large)
                VStack(alignment: .leading, spacing: 0) {
                    VStack(alignment: .leading, spacing: 0) {
                        WrappingHStack(models: viewModel.seedPhraseGrid) { gridElement in
                            switch gridElement {
                            case let .seedPhraseElement(element):
                                seedPhraseCapsule(element)
                            case .input:
                                recoveryTextInput()
                            }
                        }
                        .padding(.leading, Spacing.extraSmall)
                        .padding(.top, Spacing.extraSmall)
                        .padding(.bottom, Spacing.extraExtraSmall)
                        Spacer()
                    }
                    .frame(minHeight: 156)
                    .containerBackground(CornerRadius.small)
                    .contentShape(Rectangle())
                    .onTapGesture {
                        /// #2065 Enable to focus `recoveryTextInput` when tapping anywhere within input rectangle
                        focus = true
                    }
                    .padding(.horizontal, Spacing.large)
                    .padding(.bottom, Spacing.small)
                    ScrollView(.horizontal, showsIndicators: false) {
                        LazyHStack(alignment: .top, spacing: 0) {
                            Spacer()
                                .frame(width: Spacing.large, height: Spacing.large)
                            ForEach(viewModel.guesses, id: \.self) { guess in
                                guessCapsule(guess)
                            }
                            Spacer()
                                .frame(width: Spacing.large - Spacing.extraExtraSmall, height: Spacing.large)
                        }
                    }
                    .frame(height: 36)
                }
                NavigationLink(
                    destination:
                    CreateKeysForNetworksView(
                        viewModel: viewModel.createDerivedKeys()
                    )
                    .navigationBarHidden(true),
                    isActive: $viewModel.isPresentingDetails
                ) { EmptyView() }
            }
            .onAppear {
                focus = true
                viewModel.onAppear()
            }
            .background(Asset.backgroundPrimary.swiftUIColor)
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

    @ViewBuilder
    func seedPhraseCapsule(_ element: SeedPhraseElement) -> some View {
        HStack(alignment: .center, spacing: Spacing.extraExtraSmall) {
            Text(element.position)
                .foregroundColor(Asset.textAndIconsDisabled.swiftUIColor)
                .frame(minWidth: Sizes.seedWordPositionWidth, alignment: .trailing)
                .lineLimit(1)
                .padding(.leading, Spacing.extraSmall)
            Text(element.word)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .lineLimit(1)
                .fixedSize(horizontal: false, vertical: true)
                .padding(.trailing, Spacing.small)
        }
        .font(PrimaryFont.labelS.font)
        .frame(height: Heights.seedPhraseCapsuleHeight)
        .containerBackground()
        .padding(.bottom, Spacing.extraExtraSmall)
        .padding(.trailing, Spacing.extraExtraSmall)
    }

    @ViewBuilder
    func recoveryTextInput() -> some View {
        TextField(
            "",
            text: $viewModel.userInput
        )
        .focused($focus)
        .inlineTextFieldStyle(text: $viewModel.userInput)
        .onChange(of: viewModel.userInput, perform: { word in
            viewModel.onUserInput(word)
        })
        .frame(minWidth: 50, maxWidth: 100)
        .onAppear {
            focus = true
        }
        .padding(.bottom, Spacing.extraExtraSmall)
        .padding(.trailing, Spacing.extraExtraSmall)
    }

    @ViewBuilder
    func guessCapsule(_ guess: String) -> some View {
        Text(guess)
            .foregroundColor(Asset.accentPink300.swiftUIColor)
            .font(PrimaryFont.labelS.font)
            .padding([.top, .bottom], Spacing.extraSmall)
            .padding(.horizontal, Spacing.small)
            .background(Asset.accentPink12.swiftUIColor)
            .clipShape(Capsule())
            .onTapGesture {
                viewModel.onGuessTap(guess)
            }
            .padding(.trailing, Spacing.extraExtraSmall)
    }
}

extension RecoverKeySetSeedPhraseView {
    struct SeedPhraseElement: Equatable, Identifiable, Hashable {
        let id = UUID()
        let position: String
        let word: String
    }

    struct TextInput: Equatable, Identifiable, Hashable {
        let id = UUID()
    }

    enum GridElement: Identifiable, Hashable {
        case seedPhraseElement(SeedPhraseElement)
        case input(TextInput)

        var id: UUID {
            switch self {
            case let .seedPhraseElement(element):
                return element.id
            case let .input(input):
                return input.id
            }
        }
    }
}

extension RecoverKeySetSeedPhraseView {
    final class ViewModel: ObservableObject {
        private enum Constants {
            static let invisibleNonEmptyCharacter = "\u{200B}"
        }

        private let seedsMediator: SeedsMediating
        private let textInput = TextInput()
        private var shouldSkipUpdate = false
        private let service: RecoverKeySetService
        private let onCompletion: (CreateKeysForNetworksView.OnCompletionAction) -> Void
        private let seedName: String
        @Binding var isPresented: Bool
        @Published var isPresentingDetails: Bool = false
        @Published var isValidSeedPhrase: Bool = false
        @Published var seedPhraseGrid: [GridElement] = []
        @Published var userInput: String = Constants.invisibleNonEmptyCharacter
        @Published var guesses: [String] = []

        private var seedPhraseDraft: [String] = [] {
            didSet {
                regenerateGrid()
                validateSeedPhrase()
                userInput = Constants.invisibleNonEmptyCharacter
                updateGuesses("")
            }
        }

        private var seedPhrase: String {
            seedPhraseDraft.joined(separator: " ")
        }

        @Published var isPresentingError: Bool = false
        @Published var presentableError: ErrorBottomModalViewModel = .recoverySeedPhraseIncorrectPhrase()

        init(
            seedName: String,
            isPresented: Binding<Bool>,
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            service: RecoverKeySetService = RecoverKeySetService(),
            onCompletion: @escaping (CreateKeysForNetworksView.OnCompletionAction) -> Void
        ) {
            self.seedName = seedName
            self.seedsMediator = seedsMediator
            self.service = service
            self.onCompletion = onCompletion
            _isPresented = isPresented
            regenerateGrid()
        }

        func onAppear() {
            updateGuesses("")
        }

        func onGuessTap(_ guess: String) {
            seedPhraseDraft.append(guess)
        }

        private func updateGuesses(_ userInput: String) {
            service.updateGuessWords(userInput: userInput) { result in
                switch result {
                case let .success(guesses):
                    self.guesses = guesses
                case let .failure(error):
                    self.presentableError = .alertError(message: error.localizedDescription)
                    self.isPresentingError = true
                }
            }
        }

        func onUserInput(_ word: String) {
            guard !shouldSkipUpdate else { return }
            shouldSkipUpdate = true
            // User input is empty and invisible character was deleted
            // This means that backspace was tapped, we should delete last saved word
            if word.isEmpty {
                seedPhraseDraft = Array(seedPhraseDraft.dropLast(1))
                userInput = Constants.invisibleNonEmptyCharacter
                // User added " " while typing, we should check guess words or delete whitespace
            } else if word.hasSuffix(" ") {
                let exactWord = String(word.dropFirst().dropLast(1))
                // If there is a match, add this word and clear user input
                if guesses.contains(exactWord) {
                    seedPhraseDraft.append(exactWord)
                    // If there is no match, we should remove added whitespace
                } else {
                    userInput = exactWord
                }
                // User just added new character, generate new guesses
            } else {
                updateGuesses(String(word.dropFirst()))
            }
            shouldSkipUpdate = false
        }

        func onDoneTap() {
            isPresentingDetails = true
        }

        func createDerivedKeys() -> CreateKeysForNetworksView.ViewModel {
            .init(
                seedName: seedName,
                seedPhrase: seedPhrase,
                mode: .recoverKeySet,
                isPresented: $isPresented,
                onCompletion: onCompletion
            )
        }

        private func validateSeedPhrase() {
            service.validate(seedPhrase: seedPhrase) { result in
                switch result {
                case let .success(isValid):
                    self.isValidSeedPhrase = isValid
                case let .failure(error):
                    self.presentableError = .alertError(message: error.localizedDescription)
                    self.isPresentingError = true
                }
            }
        }
    }
}

private extension RecoverKeySetSeedPhraseView.ViewModel {
    func regenerateGrid() {
        var updatedGrid: [RecoverKeySetSeedPhraseView.GridElement] = seedPhraseDraft.enumerated()
            .map { .seedPhraseElement(.init(position: String($0.offset + 1), word: $0.element)) }
        updatedGrid.append(.input(textInput))
        seedPhraseGrid = updatedGrid
    }
}

private extension MRecoverSeedPhrase {
    func draftPhrase() -> String {
        draft.joined(separator: " ")
    }
}
