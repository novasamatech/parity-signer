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
                VStack {
                    Text("New Seed").font(.title)
                    TextField("Seed", text: $seedName, prompt: Text("Seed name"))
                        .foregroundColor(Color("textEntryColor"))
                        .background(Color("textFieldColor"))
                        .font(.largeTitle)
                        .disableAutocorrection(true)
                        .keyboardType(.asciiCapable)
                        .submitLabel(.done)
                        .onChange(of: seedName, perform: { _ in
                            data.lastError = ""
                        })
                        .onAppear(perform: {nameFocused = true})
                        .focused($nameFocused)
                        .border(Color("AccentColor"), width: 1)
                    Toggle(isOn: $recover) {
                        Text("Recover seed phrase?")
                            .font(.headline)
                            .foregroundColor(Color("textMainColor"))
                    }
                    if (recover) {
                        //TODO: make completely custom tool for this
                        TextEditor(text: $seedPhrase)
                            .frame(height: 150.0)
                            .autocapitalization(.none)
                            .keyboardType(.asciiCapable)
                            .disableAutocorrection(true)
                            .font(.title)
                            .foregroundColor(Color("cryptoColor"))
                            .background(Color("textFieldColor"))
                            .border(Color("cryptoColor"), width: /*@START_MENU_TOKEN@*/1/*@END_MENU_TOKEN@*/)
                    }
                    Text(data.lastError).foregroundColor(.red)
                    HStack{
                        Button(action: {
                            data.lastError = ""
                            seedPhrase = ""
                            data.keyManagerModal = .none
                        }) {
                            Text("Cancel").font(.largeTitle)
                        }
                        Spacer()
                        Button(action: {
                            if !recover {seedPhrase = ""}
                            data.addSeed(seedName: seedName, seedPhrase: seedPhrase)
                        }) {
                            Text("Create")
                                .font(.largeTitle)
                        }
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
