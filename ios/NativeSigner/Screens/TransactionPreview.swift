//
//  TransactionPreview.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 9.8.2021.
//

import SwiftUI

struct TransactionPreview: View {
    @State private var comment = ""
    @State private var offset: CGFloat = 0
    @State private var offsetOld: CGFloat = 0
    @FocusState private var focus: Bool
    let content: MTransaction
    let sign: (String, String) -> Void
    let navigationRequest: NavigationRequest
    var body: some View {
        VStack {
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
                        Localizable.logNote.text.font(Fontstyle.overline.base)
                            .foregroundColor(Asset.text400.swiftUIColor)
                        Spacer()
                    }
                    ZStack {
                        RoundedRectangle(cornerRadius: 8).stroke(Asset.border400.swiftUIColor).frame(height: 39)
                        TextField(
                            Localizable.comment.string,
                            text: $comment,
                            prompt: Localizable.commentNotPublished.text
                        )
                        .foregroundColor(Asset.text400.swiftUIColor)
                        .background(Asset.bg100.swiftUIColor)
                        .font(Fontstyle.body2.base)
                        .focused($focus)
                        .onDisappear {
                            focus = false
                        }
                        .padding(.horizontal, 8)
                    }
                    HStack {
                        Localizable.visibleOnlyOnThisDevice.text
                            .font(Fontstyle.subtitle1.base)
                            .padding(.bottom)
                        Spacer()
                    }
                }
                Spacer()
                VStack {
                    switch content.ttype {
                    case .sign:
                        BigButton(
                            text: Localizable.TransactionPreview.unlockSign.key,
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
                            text: Localizable.TransactionPreview.approve.key,
                            action: {
                                navigationRequest(.init(action: .goForward))
                            }
                        )
                    case .read:
                        EmptyView()
                    case .importDerivations:
                        BigButton(
                            text: Localizable.TransactionPreview.selectSeed.key,
                            isCrypto: true,
                            action: {
                                navigationRequest(.init(action: .goForward))
                            }
                        )
                    case .done:
                        EmptyView()
                    }
                    if content.ttype != .done {
                        BigButton(
                            text: Localizable.TransactionPreview.decline.key,
                            isShaded: true,
                            isDangerous: true,
                            action: {
                                focus = false
                                navigationRequest(.init(action: .goBack))
                            }
                        )
                    }
                }
            }
            .padding(.top, -10)
            .padding(.horizontal, 16)
        }
        .offset(x: 0, y: offset + offsetOld)
        .gesture(
            DragGesture()
                .onChanged { drag in
                    self.offset = drag.translation.height
                }
                .onEnded { drag in
                    self.offsetOld += drag.translation.height
                    self.offset = 0
                }
        )
    }
}

// struct TransactionPreview_Previews: PreviewProvider {
// static var previews: some View {
// TransactionPreview()
// }
// }
