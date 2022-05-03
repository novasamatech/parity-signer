//
//  RecoverSeedPhrase.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 8.12.2021.
//

import SwiftUI

struct RecoverSeedPhrase: View {
    @EnvironmentObject var data: SignerDataModel
    @State private var userInput: String = " "
    @State private var createRoots: Bool = true
    @State private var errorMessage: String? = ""
    @FocusState private var focus: Bool
    var content: MRecoverSeedPhrase
    
    var body: some View {
        ZStack {
            ScrollView {
                VStack {
                    //SeedNameCardOfSomeKind
                    Text(content.seedName.decode64())
                    VStack(alignment: .leading) {
                        Text("SEED PHRASE").font(FBase(style: .overline))
                        VStack {
                            Text(
                                content.draftPhrase()
                            )
                                .lineLimit(nil)
                                .fixedSize(horizontal: false, vertical: true)
                                .font(.system(size: 16, weight: .semibold, design: .monospaced))
                                .foregroundColor(Color("Crypto400"))
                                .padding(12)
                            Divider().foregroundColor(Color("Border400"))
                            HStack {
                                Text(">").foregroundColor(Color("Text400"))
                                    .font(FBase(style: .body2))
                                TextField("Seed", text: $userInput, prompt: Text("Seed name"))
                                    .focused($focus)
                                    .foregroundColor(Color("Text600"))
                                //.background(Color("backgroundColor"))
                                    .font(FBase(style: .body2))
                                    .disableAutocorrection(true)
                                    .textInputAutocapitalization(.never)
                                    .keyboardType(.asciiCapable)
                                    .submitLabel(.done)
                                    .onChange(of: userInput, perform: { word in
                                        data.pushButton(action: .textEntry, details: word)
                                    })
                                    .onSubmit {
                                    }
                                    .onChange(of: content, perform: { input in
                                        userInput = input.userInput
                                    })
                                    .onAppear(perform: {
                                        userInput = content.userInput
                                        focus = content.keyboard
                                    })
                                    .padding(.horizontal, 12)
                                    .padding(.top, 0)
                                    .padding(.bottom, 10)
                            }
                        }
                        .background(RoundedRectangle(cornerRadius: 8).stroke(Color("Border400")))
                        
                        ScrollView(.horizontal) {
                            LazyHStack {
                                ForEach(content.guessSet, id: \.self) { guess in
                                    VStack {
                                        Button(action: {
                                            data.pushButton(action: .pushWord, details: guess)
                                        }) {
                                            Text(guess)
                                                .foregroundColor(Color("Crypto400"))
                                                .font(FCrypto(style: .body2))
                                                .padding(.horizontal, 12)
                                                .padding(.vertical, 4)
                                                .background(RoundedRectangle(cornerRadius: 4)
                                                                .foregroundColor(Color("Crypto100")))
                                        }
                                    }
                                }
                            }
                        }.frame(height: 23)
                        
                        Spacer()
                        Text(data.lastError).foregroundColor(.red)
                        Button(action: {
                            createRoots.toggle()
                        }) {
                            HStack {
                                Image(systemName: createRoots ? "checkmark.square" : "square").imageScale(.large)
                                Text("Create seed keys")
                                    .multilineTextAlignment(.leading)
                                Spacer()
                            }
                        }
                        
                        if (!focus) {
                            HStack {
                                BigButton(
                                    text: "Next",
                                    action: {
                                        data.restoreSeed(seedName: content.seedName, seedPhrase: content.readySeed ?? "", createRoots: createRoots)
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

/*
 struct RecoverSeedPhrase_Previews: PreviewProvider {
 static var previews: some View {
 RecoverSeedPhrase()
 }
 }
 */
