//
//  SignatureReady.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 15.12.2021.
//

import SwiftUI

struct SignatureReady: View {
    @GestureState private var dragOffset = CGSize.zero
    @State private var offset: CGFloat = 0
    @State private var oldOffset: CGFloat = UIScreen.main.bounds.size.width
    var content: MSignatureReady
    let navigationRequest: NavigationRequest
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 8).foregroundColor(Asset.bg000.swiftUIColor)
            VStack {
                HeaderBar(
                    line1: Localizable.yourSignature.key,
                    line2: Localizable.scanItIntoYourApplication.key
                )
                Image(uiImage: UIImage(data: Data(content.signature)) ?? UIImage())
                    .resizable()
                    .aspectRatio(contentMode: .fit).padding(12)
                Spacer()
                BigButton(text: Localizable.done.key, action: {
                    navigationRequest(.init(action: .goBack))
                })
            }.padding(16)
        }
        .offset(x: 0, y: offset + oldOffset)
        .gesture(
            DragGesture()
                .onChanged { drag in
                    self.offset = drag.translation.height
                }
                .onEnded { drag in
                    self.oldOffset += drag.translation.height
                    self.offset = 0
                }
        )
        .gesture(
            TapGesture().onEnded { _ in
                self.oldOffset = 0
            }
        )
    }
}

// struct SignatureReady_Previews: PreviewProvider {
// static var previews: some View {
// SignatureReady()
// }
// }
