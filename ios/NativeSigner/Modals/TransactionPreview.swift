//
//  TransactionPreview.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 9.8.2021.
//

import SwiftUI

struct TransactionPreview: View {
    @EnvironmentObject var data: SignerDataModel
    @State private var comment = ""
    var body: some View {
        ZStack {
            VStack {
                switch data.action?.type {
                case "sign":
                    Text("Extrinsic to sign")
                default:
                    Text("Decoded payload")
                }
                ScrollView {
                    LazyVStack {
                        ForEach(data.cards, id: \.index) { card in
                            TransactionCardView(card: card)
                        }
                    }
                }
                Spacer()
                HStack {
                    Button(action: {data.transactionState = .none}) {
                        Text("Decline")
                            .font(.largeTitle)
                    }
                    Spacer()
                    if data.action != nil {
                        if data.action!.type == "sign_transaction" {
                            Button(action: {data.transactionState = .signed}) {
                                Text("Sign")
                                    .font(.largeTitle)
                            }
                        } else {
                            Button(action: {
                                data.signTransaction(seedPhrase: "", password: "")
                                data.totalRefresh()
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
