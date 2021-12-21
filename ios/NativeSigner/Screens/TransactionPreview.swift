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
    @FocusState private var focus: Bool
    let content: MTransaction
    var body: some View {
        ZStack {
            VStack {
                ZStack {
                    RoundedRectangle(cornerRadius: 8).stroke(Color("Crypto400"))
                ScrollView {
                    LazyVStack {
                        ForEach(content.content.assemble(), id: \.index) { card in
                            TransactionCardView(card: card)
                        }
                    }
                }
                    
                }
                if (content.type == .sign) {
                    if let address = content.author_info {
                        AddressCard(address: address.intoAddress())
                    }
                    if let network = content.network_info {
                        NetworkCard(title: network.network_title, logo: network.network_logo)
                    }
                    Text("Comment (not published)")
                    TextField("comment", text: $comment, prompt: Text("enter comment"))
                        .foregroundColor(Color("Text400"))
                        .background(Color("Bg100"))
                        .border(Color("Borders400"), width: 1)
                        .focused($focus)
                        .onDisappear {
                            focus = false
                        }
                }
                Spacer()
                HStack {
                    Button(action: {
                        focus = false
                        data.pushButton(buttonID: .GoBack)}) {
                        Text("Decline")
                            .font(.largeTitle)
                    }
                    Spacer()
                    switch content.type {
                    case .sign:
                        Button(action: {
                            focus = false
                            data.pushButton(buttonID: .GoForward, details: Data(comment.utf8).base64EncodedString(), seedPhrase: data.getSeed(seedName: content.author_info?.seed ?? ""))
                        }) {
                            Text("Sign")
                                .font(.largeTitle)
                        }
                    case .stub:
                        Button(action: {
                            data.pushButton(buttonID: .GoForward)
                        }) {
                            Text("Approve")
                                .font(.largeTitle)
                        }
                    case .read:
                        EmptyView()
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
