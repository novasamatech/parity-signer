//
//  NewIdentityScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 4.8.2021.
//

import SwiftUI

struct NewAddressScreen: View {
    
    enum Field: Hashable {
        case path
        case password
        case passwordCheck
    }
    
    @EnvironmentObject var data: SignerDataModel
    @State private var password: String = ""
    @State private var passwordCheck: String = ""
    @FocusState private var focusedField: Field?
    
    var body: some View {
        ZStack {
            ModalBackdrop()
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
                    Text("Path").foregroundColor(Color("textMainColor")).font(.footnote)
                    TextField("Path", text: $data.suggestedPath, prompt: Text("Path: //hard/soft"))
                        .foregroundColor(Color("textEntryColor"))
                        .background(Color("textFieldColor"))
                        .font(.largeTitle)
                        .disableAutocorrection(true)
                        .autocapitalization(.none)
                        .keyboardType(.asciiCapable)
                        .submitLabel(.done)
                        .onChange(of: data.suggestedPath) {path in
                            data.suggestedName = String(cString:  suggest_name(nil, path)) //this function does not fail
                            data.lastError = ""
                        }
                        .focused($focusedField, equals: .path)
                        .border(Color("AccentColor"), width: 1)
                }
                VStack (alignment: .leading) {
                    Text("Password (optional)").foregroundColor(Color("textMainColor")).font(.footnote)
                    TextField("Password", text: $password, prompt: Text("(optional)"))
                        .foregroundColor(Color("textEntryColor"))
                        .background(Color("textFieldColor"))
                        .font(.largeTitle)
                        .disableAutocorrection(true)
                        .autocapitalization(.none)
                        .keyboardType(.asciiCapable)
                        .submitLabel(.next)
                        .onChange(of: data.suggestedName, perform: {_ in data.lastError = ""
                        })
                        .focused($focusedField, equals: .password)
                        .onSubmit({if password != "" {
                            focusedField = .passwordCheck
                        }})
                        .border(Color("AccentColor"), width: 1)
                }
                if password != "" {
                    VStack (alignment: .leading) {
                        Text("Repeat password").foregroundColor(Color("textMainColor")).font(.footnote)
                        TextField("Repeat", text: $passwordCheck, prompt: Text("password"))
                            .foregroundColor(Color("textEntryColor"))
                            .background(Color("textFieldColor"))
                            .font(.largeTitle)
                            .disableAutocorrection(true)
                            .autocapitalization(.none)
                            .keyboardType(.asciiCapable)
                            .submitLabel(.done)
                            .onChange(of: data.suggestedName, perform: {_ in data.lastError = ""
                            })
                            //.focused($focusedField, equals: .passwordCheck)
                            .border(Color("AccentColor"), width: 1)
                    }}
                HStack {
                    Button(action: {
                        data.keyManagerModal = .none
                    }) {
                        Text("Cancel").font(.largeTitle)
                    }
                    Spacer()
                    Button(action: {
                        data.createAddress(password: password)
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
            focusedField = .path
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
