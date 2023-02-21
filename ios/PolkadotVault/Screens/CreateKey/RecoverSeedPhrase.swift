//
//  RecoverSeedPhrase.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 8.12.2021.
//

import SwiftUI

struct RecoverSeedPhrase: View {
    @EnvironmentObject var navigation: NavigationCoordinator
    @State private var userInput: String = " "
    @State private var shadowUserInput: String = " "
    @FocusState private var focus: Bool
    let content: MRecoverSeedPhrase

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            NavigationBarView(
                viewModel: .init(
                    title: nil,
                    leftButtons: [
                        .init(
                            type: .arrow,
                            action: { navigation.perform(navigation: .init(action: .goBack)) }
                        )
                    ],
                    rightButtons: [.init(type: .empty, action: {})]
                )
            )
            ScrollView {
                VStack(alignment: .leading, spacing: 0) {
                    Localizable.RecoverSeedPhrase.Label.title.text
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.titleL.font)
                        .padding(.top, Spacing.extraSmall)
                    Text(content.seedName)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.bodyL.font)
                        .padding(.vertical, Spacing.extraSmall)
                    Localizable.seedPhrase.text
                        .font(PrimaryFont.labelS.font)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .padding(.vertical, Spacing.extraSmall)
                    VStack(alignment: .leading, spacing: Spacing.small) {
                        VStack(alignment: .leading, spacing: 0) {
                            Text(
                                content.draftPhrase()
                            )
                            .lineLimit(nil)
                            .fixedSize(horizontal: false, vertical: true)
                            .font(.robotoMonoBold)
                            .foregroundColor(Asset.accentPink300.swiftUIColor)
                            .padding(Spacing.small)
                            HStack {
                                Spacer()
                            }
                        }
                        .strokeContainerBackground(state: .actionableInfo)
                        HStack {
                            TextField(
                                Localizable.seedName.string,
                                text: $userInput
                            )
                            .focused($focus)
                            .submitLabel(.done)
                            .primaryTextFieldStyle(Localizable.seedName.string, text: $userInput)
                            .onChange(of: userInput, perform: { word in
                                navigation.perform(navigation: .init(action: .textEntry, details: word))
                                shadowUserInput = word
                            })
                            .onSubmit {}
                            .onChange(of: shadowUserInput, perform: { _ in
                                userInput = " " + content.userInput
                            })
                            .onChange(of: content, perform: { input in
                                userInput = " " + input.userInput
                            })
                            .onAppear(perform: {
                                userInput = " " + content.userInput
                                focus = content.keyboard
                            })
                        }
                        ScrollView(.horizontal, showsIndicators: false) {
                            LazyHStack {
                                ForEach(content.guessSet, id: \.self) { guess in
                                    Text(guess)
                                        .foregroundColor(Asset.accentPink300.swiftUIColor)
                                        .font(PrimaryFont.labelS.font)
                                        .padding([.top, .bottom], Spacing.extraSmall)
                                        .padding([.leading, .trailing], Spacing.small)
                                        .background(Asset.accentPink12.swiftUIColor)
                                        .clipShape(Capsule())
                                        .onTapGesture {
                                            navigation.perform(navigation: .init(action: .pushWord, details: guess))
                                        }
                                }
                            }
                        }
                        Spacer()
                        HStack {
                            PrimaryButton(
                                action: {
                                    ServiceLocator.seedsMediator.restoreSeed(
                                        seedName: content.seedName,
                                        seedPhrase: content.readySeed ?? "",
                                        navigate: true
                                    )
                                },
                                text: Localizable.RecoverSeedPhrase.Action.recover.key,
                                style: .primary(isDisabled: .constant(content.readySeed == nil))
                            )
                            .padding(Spacing.medium)
                        }
                    }
                }
                .padding(.horizontal, Spacing.large)
            }
        }
    }
}

private extension MRecoverSeedPhrase {
    func draftPhrase() -> String {
        draft.joined(separator: " ")
    }
}

// struct RecoverSeedPhrase_Previews: PreviewProvider {
// static var previews: some View {
// RecoverSeedPhrase()
// }
// }
