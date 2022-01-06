//
//  SignatureReady.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 15.12.2021.
//

import SwiftUI

struct SignatureReady: View {
    @EnvironmentObject var data: SignerDataModel
    @GestureState private var dragOffset = CGSize.zero
    @State var offset: CGFloat = 0
    @State var oldOffset: CGFloat = UIScreen.main.bounds.size.width
    var content: MSignatureReady
    var body: some View {
        ZStack{
            //RoundedRectangle(cornerRadius: 8).foregroundColor(Color("Bg000"))
            VStack {
                HeaderBar(line1: "Your Signature", line2: "Scan it into your application")
                Image(uiImage: UIImage(data: Data(fromHexEncodedString: content.signature) ?? Data()) ?? UIImage())
                    .resizable()
                    .aspectRatio(contentMode: .fit).padding(12)
            }.padding(16)
        }.background(RoundedRectangle(cornerRadius: 8).foregroundColor(Color("Bg000")))
        .offset(x: 0, y: offset+oldOffset)
        .gesture(
            DragGesture()
                .onChanged {drag in
                    self.offset = drag.translation.height
                }
                .onEnded{drag in
                    self.oldOffset += drag.translation.height
                    self.offset = 0
                }
        )
    }
}

/*
 struct SignatureReady_Previews: PreviewProvider {
 static var previews: some View {
 SignatureReady()
 }
 }
 */
