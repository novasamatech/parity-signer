//
//  RecoverSeedPhrase.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 8.12.2021.
//

import SwiftUI

struct RecoverSeedPhrase: View {
    @State private var userInput: String = " "
    @State private var shadowUserInput: String = " "
    @FocusState private var focus: Bool
    let content: MRecoverSeedPhrase
    let restoreSeed: (String, String, Bool) -> Void
    let navigationRequest: NavigationRequest

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
                            .font(PrimaryFont.labelM.font)
                            .foregroundColor(Asset.accentPink300.swiftUIColor)
                            .padding(12)
                            Divider().foregroundColor(Asset.fill12.swiftUIColor)
                            HStack {
                                Text(">").foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                                    .font(PrimaryFont.bodyL.font)
                                TextField(Localizable.seed.string, text: $userInput, prompt: Localizable.seedName.text)
                                    .focused($focus)
                                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                                    .font(PrimaryFont.bodyL.font)
                                    .disableAutocorrection(true)
                                    .textInputAutocapitalization(.never)
                                    .keyboardType(.asciiCapable)
                                    .submitLabel(.done)
                                    .onChange(of: userInput, perform: { word in
                                        navigationRequest(.init(action: .textEntry, details: word))
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
                        ScrollView(.horizontal) {
                            LazyHStack {
                                ForEach(content.guessSet, id: \.self) { guess in
                                    VStack {
                                        Button(
                                            action: {
                                                navigationRequest(.init(action: .pushWord, details: guess))
                                            },
                                            label: {
                                                Text(guess)
                                                    .foregroundColor(Asset.accentForegroundText.swiftUIColor)
                                                    .font(PrimaryFont.captionM.font)
                                                    .padding(.horizontal, 12)
                                                    .padding(.vertical, 4)
                                                    .background(
                                                        RoundedRectangle(cornerRadius: 4)
                                                            .foregroundColor(Asset.accentPink300.swiftUIColor)
                                                    )
                                            }
                                        )
                                    }
                                }
                            }
                        }.frame(height: 23)
                        Spacer()
                        HStack {
                            BigButton(
                                text: Localizable.next.key,
                                action: {
                                    restoreSeed(content.seedName, content.readySeed ?? "", true)
                                },
                                isDisabled: content.readySeed == nil
                            )
                            .padding(.top, 16.0)
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
