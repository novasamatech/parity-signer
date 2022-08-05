//
//  TransactionPreview.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 9.8.2021.
//

import SwiftUI

struct TransactionPreview: View {
    @State private var comment = ""
    @FocusState private var focus: Bool
    let content: MTransaction
    let sign: (String, String) -> Void
    let pushButton: (Action, String, String) -> Void
    var body: some View {
        ScrollView {
            TransactionBlock(cards: content.content.assemble())
            VStack {
                if let address = content.authorInfo {
                    AddressCard(address: address)
                }
                if let network = content.networkInfo {
                    NetworkCard(title: network.networkTitle, logo: network.networkLogo)
                }
                if content.ttype == .sign {
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
                                if let seedName = content.authorInfo?.seedName {
                                    sign(seedName, comment)
                                }
                            }
                        )
                    case .stub:
                        BigButton(
                            text: "Approve",
                            action: {
                                pushButton(.goForward, "", "")
                            })
                    case .read:
                        EmptyView()
                    case .importDerivations:
                        BigButton(
                            text: "Select seed",
                            isCrypto: true,
                            action: {
                                pushButton(.goForward, "", "")
                            })
                    if content.ttype != .done {
                        BigButton(
                            text: "Decline",
                            isShaded: true,
                            isDangerous: true,
                            action: {
                                focus = false
                                pushButton(.goBack, "", "")})
                    }
                }
            }
            .padding(.top, -10)
            .padding(.horizontal, 16)
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
