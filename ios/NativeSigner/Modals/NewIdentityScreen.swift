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
                    Text("Path").font(.title).foregroundColor(Color("textMainColor"))
                TextField("//hard/soft", text: $data.suggestedPath).font(/*@START_MENU_TOKEN@*/.title/*@END_MENU_TOKEN@*/)
                        .foregroundColor(/*@START_MENU_TOKEN@*/Color("textEntryColor")/*@END_MENU_TOKEN@*/)
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("textFieldColor")/*@END_MENU_TOKEN@*/).border(/*@START_MENU_TOKEN@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/, width: /*@START_MENU_TOKEN@*/1/*@END_MENU_TOKEN@*/)
                Text("Name").font(.title).foregroundColor(Color("textMainColor"))
                TextField("Seed name", text: $data.suggestedName).font(/*@START_MENU_TOKEN@*/.title/*@END_MENU_TOKEN@*/)
                    .foregroundColor(/*@START_MENU_TOKEN@*/Color("textEntryColor")/*@END_MENU_TOKEN@*/)
                .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("textFieldColor")/*@END_MENU_TOKEN@*/).border(/*@START_MENU_TOKEN@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/, width: /*@START_MENU_TOKEN@*/1/*@END_MENU_TOKEN@*/)
                Text("Password (optional)").font(.title).foregroundColor(Color("textMainColor"))
                TextField("//hard/soft", text: $password).font(.title)
                    .foregroundColor(/*@START_MENU_TOKEN@*/Color("textEntryColor")/*@END_MENU_TOKEN@*/)
                .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("textFieldColor")/*@END_MENU_TOKEN@*/).border(/*@START_MENU_TOKEN@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/, width: /*@START_MENU_TOKEN@*/1/*@END_MENU_TOKEN@*/)
                Text("Password (repeat)").font(.title).foregroundColor(Color("textMainColor"))
                TextField("//hard/soft", text: $passwordCheck).font(/*@START_MENU_TOKEN@*/.title/*@END_MENU_TOKEN@*/)
                    .foregroundColor(/*@START_MENU_TOKEN@*/Color("textEntryColor")/*@END_MENU_TOKEN@*/)
                .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("textFieldColor")/*@END_MENU_TOKEN@*/).border(/*@START_MENU_TOKEN@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/, width: /*@START_MENU_TOKEN@*/1/*@END_MENU_TOKEN@*/)
                HStack {
                    Button(action: {
                        data.newIdentity = false
                    }) {
                        Text("Cancel").font(.largeTitle)
                    }
                    Spacer()
                    Button(action: {
                        data.createIdentity(password: password)
                        if data.lastError == "" {
                            data.newIdentity = false
                        }
                    }) {
                        Text("Create")
                            .font(.largeTitle)
                    }.disabled(password != passwordCheck)
                }
                Spacer()
            }.padding()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/).padding(.bottom, 100)
    }
}

struct NewIdentityScreen_Previews: PreviewProvider {
    static var previews: some View {
        NewIdentityScreen().previewLayout(.sizeThatFits)
    }
}
