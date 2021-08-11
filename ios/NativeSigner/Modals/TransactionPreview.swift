//
//  TransactionPreview.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 9.8.2021.
//

import SwiftUI

struct TransactionPreview: View {
    @ObservedObject var transaction: Transaction
    @Environment(\.presentationMode) var presentationMode: Binding<PresentationMode>
    var body: some View {
        ZStack {
            VStack {
                switch transaction.action?.type {
                case "sign":
                    Text("Extrinsic to sign")
                default:
                    Text("Decoded payload")
                }
                ScrollView {
                    LazyVStack {
                        ForEach(transaction.cards, id: \.index) { card in
                            TransactionCardView(card: card)
                        }
                    }
                }
                Spacer()
                HStack {
                    Button(action: {presentationMode.wrappedValue.dismiss()}) {
                        Text("Decline")
                            .font(.largeTitle)
                    }
                    Spacer()
                    if transaction.action != nil {
                        if transaction.action!.type == "sign_transaction" {
                            Button(action: {transaction.state = .show}) {
                                Text("Sign")
                                    .font(.largeTitle)
                            }
                        } else {
                            Button(action: {
                                transaction.signTransaction(seedPhrase: "", password: "")
                                presentationMode.wrappedValue.dismiss()
                            }) {
                                Text("Approve")
                                    .font(.largeTitle)
                            }
                        }
                    }
                }.padding()
            }
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
    }
}
/*
struct TransactionPreview_Previews: PreviewProvider {
    static var previews: some View {
        TransactionPreview()
    }
}
*/
