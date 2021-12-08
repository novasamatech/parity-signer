//
//  RecoverSeedPhrase.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 8.12.2021.
//

import SwiftUI

struct RecoverSeedPhrase: View {
    @EnvironmentObject var data: SignerDataModel
    @State private var seedPhrase: String = ""
    @FocusState private var nameFocused: Bool
    
    init() {
        UITextView.appearance().backgroundColor = .clear
    }
    
    var body: some View {
        ZStack{
            VStack {
                //SeedNameCardOfSomeKind
                VStack(alignment: .leading) {
                    Text("SEED PHRASE").font(FBase(style: .overline))
                    ZStack {
                        RoundedRectangle(cornerRadius: 8)
                            .stroke(Color("Borders400"))
                            //.foregroundColor(Color("Bg100"))
                            .frame(height: 150)
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
                            .font(FCrypto(style: .body1))
                            .foregroundColor(Color("Crypto400"))
                            //.background(Color("backgroundColor"))
                            .frame(height: 150)
                            .padding(8)
                    }
                    
                    Text(data.lastError).foregroundColor(.red)
                    HStack {
                        Spacer()
                        Button(action: {
                            data.pushButton(buttonID: .RecoverSeed, details: seedPhrase)
                        }) {
                            Text("Create")
                                .font(.system(size: 22))
                        }
                        .disabled(false)
                    }
                }.padding()
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
