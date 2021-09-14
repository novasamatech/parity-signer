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
    init() {
        UITextView.appearance().backgroundColor = .clear
    }
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
                }.padding()
                if !data.lastError.isEmpty {
                    Text(data.lastError)
                        .foregroundColor(.red)
                        .lineLimit(nil)
                }
                HStack {
                TextField("Path: //hard/soft", text: $data.suggestedPath)
                    .onChange(of: data.suggestedPath) {path in
                        data.suggestedName = String(cString:  suggest_name(nil, path)) //this function does not fail
                        data.lastError = ""
                    }
                    .autocapitalization(/*@START_MENU_TOKEN@*/.none/*@END_MENU_TOKEN@*/)
                    .disableAutocorrection(true)
                    .font(/*@START_MENU_TOKEN@*/.title/*@END_MENU_TOKEN@*/)
                    .foregroundColor(/*@START_MENU_TOKEN@*/Color("textEntryColor")/*@END_MENU_TOKEN@*/)
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("textFieldColor")/*@END_MENU_TOKEN@*/)
                    .border(/*@START_MENU_TOKEN@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/, width: /*@START_MENU_TOKEN@*/1/*@END_MENU_TOKEN@*/)
                }
                HStack {
                TextField("Seed name", text: $data.suggestedName)
                    .onChange(of: data.suggestedName, perform: {_ in data.lastError = ""
                    })
                    .font(/*@START_MENU_TOKEN@*/.title/*@END_MENU_TOKEN@*/)
                    .foregroundColor(/*@START_MENU_TOKEN@*/Color("textEntryColor")/*@END_MENU_TOKEN@*/)
                .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("textFieldColor")/*@END_MENU_TOKEN@*/).border(/*@START_MENU_TOKEN@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/, width: /*@START_MENU_TOKEN@*/1/*@END_MENU_TOKEN@*/)
                }
                HStack {
                TextField("optional password", text: $password)
                    .onChange(of: data.suggestedName, perform: {_ in data.lastError = ""
                    })
                    .font(.title)
                    .autocapitalization(.none)
                    .disableAutocorrection(true)
                    .foregroundColor(/*@START_MENU_TOKEN@*/Color("textEntryColor")/*@END_MENU_TOKEN@*/)
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("textFieldColor")/*@END_MENU_TOKEN@*/)
                    .border(/*@START_MENU_TOKEN@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/, width: /*@START_MENU_TOKEN@*/1/*@END_MENU_TOKEN@*/)
                }
                if password != "" {
                HStack {
                TextField("password(again)", text: $passwordCheck)
                    .onChange(of: data.suggestedName, perform: {_ in data.lastError = ""
                    })
                    .font(/*@START_MENU_TOKEN@*/.title/*@END_MENU_TOKEN@*/)
                    .autocapitalization(.none)
                    .disableAutocorrection(true)
                    .foregroundColor(/*@START_MENU_TOKEN@*/Color("textEntryColor")/*@END_MENU_TOKEN@*/)
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("textFieldColor")/*@END_MENU_TOKEN@*/)
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
