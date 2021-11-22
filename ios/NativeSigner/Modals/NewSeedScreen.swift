//
//  NewSeedScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 28.7.2021.
//

import SwiftUI

struct NewSeedScreen: View {
    
    @EnvironmentObject var data: SignerDataModel
    @State private var seedName: String = ""
    @State private var seedPhrase: String = ""
    @State private var recover: Bool = false
    @FocusState private var nameFocused: Bool
    
    init() {
        UITextView.appearance().backgroundColor = .clear
    }
    
    var body: some View {
        ZStack{
            ModalBackdrop()
            VStack {
                VStack(alignment: .leading) {
                    Text("New Seed").font(.title)
                    Text("DISPLAY NAME").font(.callout)
                    ZStack {
                        RoundedRectangle(cornerRadius: 8).stroke(Color("AccentColor")).foregroundColor(Color("backgroundColor")).frame(height: 39)
                    TextField("Seed", text: $seedName, prompt: Text("Seed name"))
                        .focused($nameFocused)
                        .foregroundColor(Color("textEntryColor"))
                        .background(Color("backgroundColor"))
                        .font(.system(size: 16, weight: .regular))
                        .disableAutocorrection(true)
                        .keyboardType(.asciiCapable)
                        .submitLabel(.done)
                        .onChange(of: seedName, perform: { _ in
                            data.lastError = ""
                        })
                        .onAppear(perform: {nameFocused = true})
                        .padding(.horizontal, 8)
                    }
                    Text("Display name visible only to you").font(.callout)
                    Toggle(isOn: $recover) {
                        Text("Recover seed phrase?")
                            .font(.headline)
                            .foregroundColor(Color("textMainColor"))
                    }
                    if (recover) {
                        ZStack {
                            RoundedRectangle(cornerRadius: 8).stroke(Color("AccentColor")).foregroundColor(Color("backgroundColor")).frame(height: 150)
                        //TODO: make completely custom tool for this
                        TextEditor(text: $seedPhrase)
                                .onChange(of: seedPhrase, perform: { _ in
                                    if seedPhrase.contains("\n") {
                                        seedPhrase = seedPhrase.replacingOccurrences(of: "\n", with: "")
                                        nameFocused = true
                                    }
                                })
                            .autocapitalization(.none)
                            .keyboardType(.asciiCapable)
                            .disableAutocorrection(true)
                            .font(.system(size: 16, design: .monospaced))
                            .foregroundColor(Color("cryptoColor"))
                            .background(Color("backgroundColor"))
                            .frame(height: 150)
                            .padding(8)
                        }
                    }
                    Text(data.lastError).foregroundColor(.red)
                    HStack {
                        Spacer()
                        Button(action: {
                            if !recover {seedPhrase = ""}
                            data.addSeed(seedName: seedName, seedPhrase: seedPhrase)
                        }) {
                            Text("Create")
                                .font(.system(size: 22))
                        }
                        .disabled(seedName == "")
                    }
                }.padding()
            }
        }
    }
}

/*
 struct NewSeedScreen_Previews: PreviewProvider {
 static var previews: some View {
 NewSeedScreen().previewLayout(.sizeThatFits)
 }
 }
 */
