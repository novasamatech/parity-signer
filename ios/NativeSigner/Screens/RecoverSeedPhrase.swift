//
//  RecoverSeedPhrase.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 8.12.2021.
//

import SwiftUI

struct RecoverSeedPhrase: View {
    @EnvironmentObject var data: SignerDataModel
    @State private var seedPhrase: [String] = []
    @State private var seedWord: String = ">"
    @State private var guessWord: [String] = []
    @FocusState private var focus: Bool
    let allowedLendth = [12, 24]
    
    init() {
        UITextView.appearance().backgroundColor = .clear
    }
    
    var body: some View {
        ZStack{
            VStack {
                //SeedNameCardOfSomeKind
                Text("Seedname something")
                VStack(alignment: .leading) {
                    Text("SEED PHRASE").font(FBase(style: .overline))
                    ZStack {
                        RoundedRectangle(cornerRadius: 8).stroke(Color("Crypto400")).foregroundColor(Color("Bg000"))
                        Text(seedPhrase.joined(separator: " "))
                            .lineLimit(nil)
                            .fixedSize(horizontal: false, vertical: true)
                            .font(.system(size: 16, weight: .semibold, design: .monospaced))
                            .foregroundColor(Color("Crypto400"))
                            .padding(8)
                    }
                    ZStack {
                        RoundedRectangle(cornerRadius: 8).stroke(Color("Borders400")).foregroundColor(Color("Borders400")).frame(height: 39)
                        TextField("Seed", text: $seedWord, prompt: Text("Seed name"))
                            .focused($focus)
                            .foregroundColor(Color("Text600"))
                        //.background(Color("backgroundColor"))
                            .font(FBase(style: .body2))
                            .disableAutocorrection(true)
                            .keyboardType(.asciiCapable)
                            .submitLabel(.done)
                            .onChange(of: seedWord, perform: { word in
                                data.lastError = ""
                                if word == "" {
                                    if seedPhrase.count > 0 {
                                        seedPhrase.removeLast()
                                    }
                                    seedWord = ">"
                                    guessWord = data.guessWord(word: "")
                                } else {
                                    guessWord = data.guessWord(word: String(seedWord.dropFirst()))
                                    if guessWord.count == 1 {
                                        seedPhrase.append(guessWord.popLast()!)
                                        seedWord = ">"
                                        guessWord = data.guessWord(word: "")
                                    }
                                }
                            })
                            .onSubmit {
                            }
                            .onAppear(perform: {
                                guessWord = data.guessWord(word: "")
                                focus = true
                            })
                            .padding(.horizontal, 8)
                            .onDisappear {
                                focus = false
                            }
                    }
                    ScrollView(.horizontal) {
                        LazyHStack {
                            ForEach(guessWord, id: \.self) { guess in
                                VStack {
                                    Button(action: {
                                        seedPhrase.append(guess)
                                        seedWord = ">"
                                        guessWord = data.guessWord(word: "")
                                    }) {
                                        Text(guess)
                                    }
                                }
                            }
                        }
                    }
                    Text(data.lastError).foregroundColor(.red)
                    if (!focus) {
                    HStack {
                        Button(action: {
                            data.restoreSeed(seedName: "test", seedPhrase: seedPhrase.joined(separator: " "))
                        }) {
                            Text("Create")
                                .font(.system(size: 22))
                        }
                        .disabled(!allowedLendth.contains(seedPhrase.count))
                    }
                    }
                }.padding(.horizontal)
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
