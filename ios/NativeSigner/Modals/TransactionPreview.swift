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
                ScrollView {
                    LazyVStack {
                        ForEach(data.cards, id: \.index) { card in
                            TransactionCardView(card: card)
                        }
                    }
                    if (data.action?.type == "sign") {
                        TransactionCommentInput()
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
                        if data.action!.type == "sign" {
                            Button(action: {data.transactionState = .signed}) {
                                Text("Sign")
                                    .font(.largeTitle)
                            }
                        } else {
                            Button(action: {
                                data.handleTransaction()
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
        .onAppear{
            print(data.cards)
        }
    }
}
/*
struct TransactionPreview_Previews: PreviewProvider {
    static var previews: some View {
        TransactionPreview()
    }
}
*/
