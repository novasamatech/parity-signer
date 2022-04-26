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
                if let address = content.authorInfo {
                    AddressCard(address: address.toAddress())
                }
                if let network = content.networkInfo {
                    NetworkCard(title: network.networkTitle, logo: network.networkLogo)
                }
                if (content.ttype == .sign) {
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
                    switch content.ttype {
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
                                        action: .goForward,
                                        details: Data(comment.utf8).base64EncodedString(),
                                        seedPhrase: data.getSeed(seedName: content.authorInfo?.seed ?? "")
                                    )
                                }
                            }
                        )
                    case .stub:
                        BigButton(
                            text: "Approve",
                            action: {
                                data.pushButton(action: .goForward)
                            })
                    case .read:
                        EmptyView()
                    case .importDerivations:
                        BigButton(
                            text: "Select seed",
                            isCrypto: true,
                            action: {
                                data.pushButton(action: .goForward)
                            })
                    case .done:
                        EmptyView()
                    }
                    if content.ttype != .done {
                        BigButton(
                            text: "Decline",
                            isShaded: true,
                            isDangerous: true,
                            action: {
                                focus = false
                                data.pushButton(action: .goBack)})
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
