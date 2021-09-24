//
//  NewIdentityScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 4.8.2021.
//

import SwiftUI

struct NewIdentityScreen: View {
    @EnvironmentObject var data: SignerDataModel
    @State private var password: String = ""
    @State private var passwordCheck: String = ""
    @State private var pathFocus = true
    @State private var nameFocus = false
    @State private var passwordFocus = false
    @State private var passwordCheckFocus = false
    
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 50).foregroundColor(/*@START_MENU_TOKEN@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
            VStack {
                Text("New key").font(.title).foregroundColor(Color("AccentColor"))
                HStack {
                    NetworkCard(network: data.selectedNetwork)
                    Spacer()
                    VStack {
                        Text("seed").font(.footnote)
                        Text(data.selectedSeed)
                            .font(.headline)
                    }
                }
                if !data.lastError.isEmpty {
                    Text(data.lastError)
                        .foregroundColor(.red)
                        .lineLimit(nil)
                }
                VStack (alignment: .leading) {
                    Text("path").foregroundColor(Color("textMainColor")).font(.footnote)
                    SignerTextInput(text: $data.suggestedPath, focus: $pathFocus, placeholder: "Path: //hard/soft", autocapitalization: .none, returnKeyType: .done, keyboardType: .asciiCapable, onReturn: {})
                        .onChange(of: data.suggestedPath) {path in
                            data.suggestedName = String(cString:  suggest_name(nil, path)) //this function does not fail
                            data.lastError = ""
                        }
                        .border(/*@START_MENU_TOKEN@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/, width: /*@START_MENU_TOKEN@*/1/*@END_MENU_TOKEN@*/)
                }
                VStack (alignment: .leading) {
                    Text("key name").foregroundColor(Color("textMainColor")).font(.footnote)
                    SignerTextInput(text: $data.suggestedName, focus: $nameFocus, placeholder: "Seed name", autocapitalization: .none, returnKeyType: .done, keyboardType: .default, onReturn: {})
                        .onChange(of: data.suggestedName, perform: {_ in data.lastError = ""
                        })
                        .border(/*@START_MENU_TOKEN@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/, width: /*@START_MENU_TOKEN@*/1/*@END_MENU_TOKEN@*/)
                }
                VStack (alignment: .leading) {
                    Text("password").foregroundColor(Color("textMainColor")).font(.footnote)
                    SignerTextInput(text: $password, focus: $passwordFocus, placeholder: "password (optional)", autocapitalization: .none, returnKeyType: .next, keyboardType: .asciiCapable, onReturn: {
                        if password != "" {
                            passwordCheckFocus = true
                        }
                    })
                    .onChange(of: data.suggestedName, perform: {_ in data.lastError = ""
                    })
                    .border(/*@START_MENU_TOKEN@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/, width: /*@START_MENU_TOKEN@*/1/*@END_MENU_TOKEN@*/)
                }
                if password != "" {
                    VStack (alignment: .leading) {
                        Text("password (repeat)").foregroundColor(Color("textMainColor")).font(.footnote)
                        SignerTextInput(text: $passwordCheck, focus: $passwordCheckFocus, placeholder: "Repeat password", autocapitalization: .none, returnKeyType: .done, keyboardType: .asciiCapable, onReturn: {
                            if password == passwordCheck {
                                data.createIdentity(password: password)
                                if data.lastError == "" {
                                    data.keyManagerModal = .none
                                }
                            }
                        })
                        .onChange(of: data.suggestedName, perform: {_ in data.lastError = ""
                        })
                        .border(/*@START_MENU_TOKEN@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/, width: /*@START_MENU_TOKEN@*/1/*@END_MENU_TOKEN@*/)
                    }}
                HStack {
                    Button(action: {
                        data.keyManagerModal = .none
                    }) {
                        Text("Cancel").font(.largeTitle)
                    }
                    Spacer()
                    Button(action: {
                        data.createIdentity(password: password)
                        if data.lastError == "" {
                            data.keyManagerModal = .none
                        }
                    }) {
                        Text("Create")
                            .font(.largeTitle)
                    }
                    .disabled(password != passwordCheck)
                }
            }.padding(.horizontal)
        }
        .onAppear {
            data.lastError = ""
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
