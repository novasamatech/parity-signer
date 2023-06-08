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
                    title: Localizable.RecoverSeedPhrase.Label.title.string,
                    leftButtons: [.init(type: .arrow, action: { mode.wrappedValue.dismiss() })],
                    rightButtons: [.init(
                        type: .activeAction(
                            Localizable.RecoverSeedPhrase.Action.done.key,
                            .constant(viewModel.content.readySeed == nil)
                        ),
                        action: viewModel.onDoneTap
                    )]
                )
            )
            ScrollView(showsIndicators: false) {
                VStack(alignment: .leading, spacing: 0) {
                    Localizable.RecoverSeedPhrase.Label.header.text
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.bodyL.font)
                        .padding(.leading, Spacing.large)
                        .padding(.top, Spacing.large)
                        .padding(.bottom, Spacing.small)
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
                    .padding(.horizontal, Spacing.large)
                    .padding(.bottom, Spacing.small)
                    ScrollView(.horizontal, showsIndicators: false) {
                        LazyHStack(alignment: .top, spacing: 0) {
                            Spacer()
                                .frame(width: Spacing.large, height: Spacing.large)
                            ForEach(viewModel.content.guessSet, id: \.self) { guess in
                                guessCapsule(guess)
                            }
                            Spacer()
                                .frame(width: Spacing.large - Spacing.extraExtraSmall, height: Spacing.large)
                        }
                    }
                    .frame(height: 36)
                }
            }
            .onAppear {
                focus = true
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
            NavigationLink(
                destination:
                CreateKeysForNetworksView(
                    viewModel: viewModel.createDerivedKeys()
                )
                .navigationBarHidden(true),
                isActive: $viewModel.isPresentingDetails
            ) { EmptyView() }
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
            .padding([.leading, .trailing], Spacing.small)
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
        private let seedsMediator: SeedsMediating
        private let textInput = TextInput()
        private var shouldSkipUpdate = false
        private let service: RecoverKeySetService
        @Binding var isPresented: Bool
        @Published var isPresentingDetails: Bool = false
        @Published var seedPhraseGrid: [GridElement] = []
        @Published var userInput: String = " "
        @Published var previousUserInput: String = " "

        @Published var isPresentingError: Bool = false
        @Published var presentableError: ErrorBottomModalViewModel = .recoverySeedPhraseIncorrectPhrase()

        var content: MRecoverSeedPhrase {
            didSet {
                regenerateGrid()
                shouldPresentError()
            }
        }

        init(
            content: MRecoverSeedPhrase,
            isPresented: Binding<Bool>,
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            service: RecoverKeySetService = RecoverKeySetService()
        ) {
            self.content = content
            self.seedsMediator = seedsMediator
            self.service = service
            _isPresented = isPresented
            regenerateGrid()
        }

        func onGuessTap(_ guess: String) {
            guard let updatedContent = service.updateGuess(guess) else { return }
            content = updatedContent
            userInput = " "
        }

        func onUserInput(_ word: String) {
            guard !shouldSkipUpdate else { return }
            shouldSkipUpdate = true
            let wordToSend = word.isEmpty && !previousUserInput.isEmpty ? " " : word
            guard let updatedContent = service.onUserEntry(wordToSend) else { return }
            content = updatedContent
            if content.userInput != userInput {
                userInput = content.userInput
            }
            previousUserInput = userInput
            if userInput.isEmpty, content.userInput.isEmpty {
                userInput = " "
            }
            shouldSkipUpdate = false
        }

        func onDoneTap() {
            let seedPhrase = content.readySeed ?? ""
            if seedsMediator.checkSeedPhraseCollision(seedPhrase: seedPhrase) {
                presentableError = .seedPhraseAlreadyExists()
                isPresentingError = true
                return
            }
            seedsMediator.createSeed(
                seedName: content.seedName,
                seedPhrase: seedPhrase,
                shouldCheckForCollision: false
            )
            service.finishKeySetRecover(seedPhrase)
            isPresentingDetails = true
        }

        func createDerivedKeys() -> CreateKeysForNetworksView.ViewModel {
            .init(seedName: content.seedName, mode: .recoverKeySet, isPresented: $isPresented)
        }
    }
}

private extension RecoverKeySetSeedPhraseView.ViewModel {
    func regenerateGrid() {
        var updatedGrid: [RecoverKeySetSeedPhraseView.GridElement] = content.draft.enumerated()
            .map { .seedPhraseElement(.init(position: String($0.offset + 1), word: $0.element)) }
        updatedGrid.append(.input(textInput))
        seedPhraseGrid = updatedGrid
    }

    func shouldPresentError() {
        isPresentingError = content.draft.count == 24 && (content.readySeed?.isEmpty ?? true)
    }
}

private extension MRecoverSeedPhrase {
    func draftPhrase() -> String {
        draft.joined(separator: " ")
    }
}
