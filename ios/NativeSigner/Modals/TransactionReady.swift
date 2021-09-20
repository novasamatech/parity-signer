//
//  TransactionReady.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 10.8.2021.
//

import SwiftUI

struct TransactionReady: View {
    @EnvironmentObject var data: SignerDataModel
    @State var seedPhrase: String = ""
    @State var password: String = ""
    @State var passwordFocus: Bool = true
    var body: some View {
        ZStack {
            VStack {
                if !data.transactionError.isEmpty {
                    Text(data.transactionError)
                        .font(.subheadline)
                        .foregroundColor(Color("AccentColor"))
                }
                if data.qr != nil {
                    VStack {
                        Text("Scan to publish")
                            .font(.largeTitle)
                            .foregroundColor(Color("textMainColor"))
                        Image(uiImage: data.qr!)
                            .resizable()
                            .aspectRatio(contentMode: .fit)
                        Text("Signed by:")
                            .foregroundColor(Color("AccentColor"))
                        Text(data.author?.name ?? "unknown")
                            .foregroundColor(Color("textMainColor"))
                        Spacer()
                        Button(action: {
                            data.qr = nil
                            data.totalRefresh()
                        }) {
                            Text("Done")
                                .font(.largeTitle)
                                
                        }
                    }
                } else {
                    //TODO: move to another screen
                    Text("Enter password")
                        .font(.body)
                        .foregroundColor(Color("textMainColor"))
                    SignerTextInput(text: $password, focus: $passwordFocus, placeholder: "password (optional)", autocapitalization: .none, returnKeyType: .done, keyboardType: .default, onReturn: {
                        data.signTransaction(seedPhrase: seedPhrase, password: password)
                    }).border(/*@START_MENU_TOKEN@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/, width: /*@START_MENU_TOKEN@*/1/*@END_MENU_TOKEN@*/)
                    Spacer()
                    HStack {
                        Button(action: {
                            data.totalRefresh()
                        }) {
                            Text("Cancel")
                                .font(.largeTitle)
                                .foregroundColor(Color("textMainColor"))
                        }
                        Button(action: {
                            data.signTransaction(seedPhrase: seedPhrase, password: password)
                        }) {
                            Text("Submit")
                                .font(.largeTitle)
                                .foregroundColor(Color("textMainColor"))
                        }
                    }
                }
                
            }
        }
        .onAppear {
            seedPhrase = data.getSeed(seedName: data.author!.seed) //this should not even be called if author is not present, so crash here
            //TODO: maybe graceful crash
            if data.author?.has_password == false {
                data.signTransaction(seedPhrase: seedPhrase, password: password)
            }
        }
        .onDisappear {
            seedPhrase = "" // TODO: It is an overkill, but is this good enough?
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct TransactionReady_Previews: PreviewProvider {
    static var previews: some View {
        TransactionReady()
    }
}
*/
