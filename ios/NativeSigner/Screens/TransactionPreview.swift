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
    let content: TransactionCardSet
    var body: some View {
        ZStack {
            VStack {
                ScrollView {
                    LazyVStack {
                        ForEach(content.assemble(), id: \.index) { card in
                            TransactionCardView(card: card)
                        }
                    }
                }
                if (content.action?.type == "sign") {
                    if let address = content.getAuthor()?.intoAddress() {
                        AddressCard(address: address)
                    }
                    Text("Comment (not published)")
                    TextField("comment", text: $comment, prompt: Text("enter comment"))
                        .foregroundColor(Color("Text400"))
                        .background(Color("Bg100")).border(Color("Borders400"), width: 1)
                }
                Spacer()
                HStack {
                    Button(action: {data.pushButton(buttonID: .GoBack)}) {
                        Text("Decline")
                            .font(.largeTitle)
                    }
                    Spacer()
                    if data.action != nil {
                        if data.action!.type == "sign" {
                            Button(action: {
                                data.pushButton(buttonID: .GoForward, details: comment, seedPhrase: data.getSeed(seedName: content.getAuthor()?.seed ?? ""))
                            }) {
                                Text("Sign")
                                    .font(.largeTitle)
                            }
                        } else {
                            Button(action: {
                                data.pushButton(buttonID: .GoForward)
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
