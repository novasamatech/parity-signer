//
//  NewIdentityScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 4.8.2021.
//

import SwiftUI

struct NewAddressScreen: View {
    
    @EnvironmentObject var data: SignerDataModel
    @State var derivationState = DerivationState(isValid: true, hasPassword: false)
    @State var path: String = ""
    @FocusState private var focusedField: Bool
    
    var content: MDeriveKey
    
    var body: some View {
        ZStack {
            ScrollView {
                //SeedCardForManager(seedName: data.selectedSeed)
                NetworkCard(title: content.network_title, logo: content.network_logo)
                if !data.lastError.isEmpty {
                    Text(data.lastError)
                        .foregroundColor(.red)
                        .lineLimit(nil)
                }
                VStack (alignment: .leading) {
                    Text("DERIVATION PATH").foregroundColor(Color("Text500")).font(.footnote)
                    ZStack {
                        RoundedRectangle(cornerRadius: 8).stroke(Color("Crypto400")).frame(height: 39)
                        HStack {
                            Text(content.seed_name)
                            TextField("Path", text: $path, prompt: Text("//<network>//input"))
                                .foregroundColor(Color("Crypto400"))
                                .font(.system(size: 15, design: .monospaced))
                                .disableAutocorrection(true)
                                .autocapitalization(.none)
                                .keyboardType(.asciiCapable)
                                .submitLabel(.done)
                                .onChange(of: path) {pathNew in
                                    derivationState = pathNew.checkAsDerivation()
                                }
                                .focused($focusedField)
                                .padding(8)
                        }
                    }
                }.padding()
                HStack {
                    Button(action: {
                        if derivationState.hasPassword {
                            data.pushButton(buttonID: .CheckPassword, details: path)
                        } else {
                            data.createAddress(path: path, seedName: content.seed_name)
                        }
                    }) {
                        Text("Next")
                    }
                    .disabled(!derivationState.isValid)
                }
            }.padding(.horizontal)
        }
        .onAppear {
            path = content.suggested_derivation
            derivationState = path.checkAsDerivation()
            focusedField = true
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
    }
}

/*
 struct NewIdentityScreen_Previews: PreviewProvider {
 static var previews: some View {
 NewIdentityScreen().previewLayout(.sizeThatFits)
 }
 }
 */
