//
//  RecoverSeedPhrase.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 8.12.2021.
//

import SwiftUI

struct RecoverSeedPhrase: View {
    @State private var userInput: String = " "
    @State private var createRoots: Bool = true
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
                        Text("SEED PHRASE").font(Fontstyle.overline.base)
                        VStack {
                            Text(
                                content.draftPhrase()
                            )
                            .lineLimit(nil)
                            .fixedSize(horizontal: false, vertical: true)
                            .font(.system(size: 16, weight: .semibold, design: .monospaced))
                            .foregroundColor(Asset.crypto400.swiftUIColor)
                            .padding(12)
                            Divider().foregroundColor(Asset.border400.swiftUIColor)
                            HStack {
                                Text(">").foregroundColor(Asset.text400.swiftUIColor)
                                    .font(Fontstyle.body2.base)
                                TextField("Seed", text: $userInput, prompt: Text("Seed name"))
                                    .focused($focus)
                                    .foregroundColor(Asset.text600.swiftUIColor)
                                    .font(Fontstyle.body2.base)
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
                        .background(RoundedRectangle(cornerRadius: 8).stroke(Asset.border400.swiftUIColor))
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
                                                    .foregroundColor(Asset.crypto400.swiftUIColor)
                                                    .font(Fontstyle.body2.crypto)
                                                    .padding(.horizontal, 12)
                                                    .padding(.vertical, 4)
                                                    .background(
                                                        RoundedRectangle(cornerRadius: 4)
                                                            .foregroundColor(Asset.crypto100.swiftUIColor)
                                                    )
                                            }
                                        )
                                    }
                                }
                            }
                        }.frame(height: 23)
                        Spacer()
                        Button(
                            action: {
                                createRoots.toggle()
                            },
                            label: {
                                HStack {
                                    Image(systemName: createRoots ? "checkmark.square" : "square").imageScale(.large)
                                    Text("Create root keys")
                                        .multilineTextAlignment(.leading)
                                    Spacer()
                                }
                            }
                        )
                        if !focus {
                            HStack {
                                BigButton(
                                    text: "Next",
                                    action: {
                                        restoreSeed(content.seedName, content.readySeed ?? "", createRoots)
                                    },
                                    isDisabled: content.readySeed == nil
                                )
                                .padding(.top, 16.0)
                            }
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
