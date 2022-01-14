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
    @State var offset: CGFloat = 0
    @State var offsetOld: CGFloat = 0
    @FocusState private var focus: Bool
    let content: MTransaction
    var body: some View {
        VStack {
            TransactionBlock(cards: content.content.assemble())
            VStack {
                if let address = content.author_info {
                    AddressCard(address: address.intoAddress())
                }
                if let network = content.network_info {
                    NetworkCard(title: network.network_title, logo: network.network_logo)
                }
                if (content.type == .sign) {
                    HStack {
                        Text("LOG NOTE").font(FBase(style: .overline)).foregroundColor(Color("Text400"))
                        Spacer()
                    }
                    ZStack {
                        RoundedRectangle(cornerRadius: 8).stroke(Color("Border400")).frame(height: 39)
                        TextField("comment", text: $comment, prompt: Text("Comment (not published)"))
                            .foregroundColor(Color("Text400"))
                            .background(Color("Bg100"))
                            .font(FBase(style: .body2))
                        //.border(Color("Border400"), width: 1)
                            .focused($focus)
                            .onDisappear {
                                focus = false
                            }
                            .padding(.horizontal, 8)
                    }
                    HStack {
                        Text("visible only on this device").font(FBase(style: .subtitle1))
                            .padding(.bottom)
                        Spacer()
                    }
                }
                Spacer()
                VStack {
                    switch content.type {
                    case .sign:
                        BigButton(
                            text: "Unlock key and sign",
                            isShaded: false,
                            isCrypto: true,
                            action: {
                                focus = false
                                if data.alert {
                                    data.alertShow = true
                                } else {
                                    data.pushButton(
                                        buttonID: .GoForward,
                                        details: Data(comment.utf8).base64EncodedString(),
                                        seedPhrase: data.getSeed(seedName: content.author_info?.seed ?? "")
                                    )
                                }
                            }
                        )
                    case .stub:
                        BigButton(
                            text: "Approve",
                            action: {
                                data.pushButton(buttonID: .GoForward)
                            })
                    case .read:
                        EmptyView()
                    case .import_derivations:
                        BigButton(
                            text: "Select seed",
                            isCrypto: true,
                            action: {
                                data.pushButton(buttonID: .GoForward)
                            })
                    case .done:
                        EmptyView()
                    }
                    if content.type != .done {
                        BigButton(
                            text: "Decline",
                            isShaded: true,
                            isDangerous: true,
                            action: {
                                focus = false
                                data.pushButton(buttonID: .GoBack)})
                    }
                }
            }
            .padding(.top, -10)
            .padding(.horizontal, 16)
        }
        .offset(x:0, y: offset+offsetOld)
        .gesture(DragGesture()
                    .onChanged{ drag in
            self.offset = drag.translation.height
        }
                    .onEnded { drag in
            self.offsetOld += drag.translation.height
            self.offset = 0
        })
    }
}

/*
 struct TransactionPreview_Previews: PreviewProvider {
 static var previews: some View {
 TransactionPreview()
 }
 }
 */
