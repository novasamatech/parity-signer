//
//  TransactionReady.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 10.8.2021.
//

import SwiftUI

struct TransactionReady: View {
    @EnvironmentObject var data: SignerDataModel
    @ObservedObject var transaction: Transaction
    @Environment(\.presentationMode) var presentationMode: Binding<PresentationMode>
    @State var seedPhrase: String = ""
    @State var password: String = ""
    var body: some View {
        ZStack {
            VStack {
                if !transaction.transactionError.isEmpty {
                    Text(transaction.transactionError)
                        .font(.subheadline)
                        .foregroundColor(Color("AccentColor"))
                }
                if transaction.qr != nil {
                    VStack {
                        Text("Scan to publish")
                            .font(.largeTitle)
                            .foregroundColor(Color("textMainColor"))
                        Image(uiImage: transaction.qr!)
                            .resizable()
                            .aspectRatio(contentMode: .fit)
                        Text("Signed by:")
                            .foregroundColor(Color("AccentColor"))
                        Text(transaction.author?.name ?? "unknown")
                            .foregroundColor(Color("textMainColor"))
                        Spacer()
                        Button(action: {
                            transaction.qr = nil
                            presentationMode.wrappedValue.dismiss()
                        }) {
                            Text("Done")
                                .font(.largeTitle)
                                
                        }
                    }
                } else {
                    Text("Enter password")
                        .font(.body)
                        .foregroundColor(Color("textMainColor"))
                    TextField("password", text: $password).font(.title)
                        .foregroundColor(/*@START_MENU_TOKEN@*/Color("textEntryColor")/*@END_MENU_TOKEN@*/)
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("textFieldColor")/*@END_MENU_TOKEN@*/).border(/*@START_MENU_TOKEN@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/, width: /*@START_MENU_TOKEN@*/1/*@END_MENU_TOKEN@*/)
                    Spacer()
                    HStack {
                        Button(action: {
                            presentationMode.wrappedValue.dismiss()
                        }) {
                            Text("Cancel")
                                .font(.largeTitle)
                                .foregroundColor(Color("textMainColor"))
                        }
                        Button(action: {
                                transaction.signTransaction(seedPhrase: seedPhrase, password: password)
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
            seedPhrase = data.getSeed(seedName: transaction.author!.seed) //this should not even be called if author is not present, so crash here
            //TODO: maybe graceful crash
            if transaction.author?.has_password == false {
                transaction.signTransaction(seedPhrase: seedPhrase, password: password)
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
