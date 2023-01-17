//
//  RecoverSeedPhrase.swift
//  NativeSigner
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
        ZStack {
            ScrollView {
                VStack {
                    Text(content.seedName)
                    VStack(alignment: .leading) {
                        Localizable.seedPhrase.text.font(PrimaryFont.labelS.font)
                        VStack {
                            Text(
                                content.draftPhrase()
                            )
                            .lineLimit(nil)
                            .fixedSize(horizontal: false, vertical: true)
                            .font(.robotoMonoBold)
                            .foregroundColor(Asset.accentPink300.swiftUIColor)
                            .padding(12)
                            Divider().foregroundColor(Asset.fill12.swiftUIColor)
                            HStack {
                                Text(">").foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                                    .font(.robotoMonoBold)
                                TextField(Localizable.seed.string, text: $userInput, prompt: Localizable.seedName.text)
                                    .focused($focus)
                                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                                    .font(PrimaryFont.bodyL.font)
                                    .disableAutocorrection(true)
                                    .textInputAutocapitalization(.never)
                                    .keyboardType(.asciiCapable)
                                    .submitLabel(.done)
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
                                    .padding(.horizontal, 12)
                                    .padding(.top, 0)
                                    .padding(.bottom, 10)
                            }
                        }
                        .background(RoundedRectangle(cornerRadius: 8).stroke(Asset.fill12.swiftUIColor))
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
                                text: Localizable.next.key,
                                style: .primary(isDisabled: .constant(content.readySeed == nil))
                            )
                            .padding(Spacing.medium)
                        }
                    }.padding(.horizontal)
                }
            }
        }
    }
}

// struct RecoverSeedPhrase_Previews: PreviewProvider {
// static var previews: some View {
// RecoverSeedPhrase()
// }
// }
