//
//  SignatureReady.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 15.12.2021.
//

import SwiftUI

struct SignatureReady: View {
    var content: MSignatureReady
    let pushButton: (Action, String, String) -> Void
    var body: some View {
        ScrollViewReader { scrollView in
            ScrollView {
                TransactionBlock(cards: content.content.assemble())
                AddressCard(address: content.authorInfo)
                NetworkCard(title: content.networkInfo.networkTitle, logo: content.networkInfo.networkLogo)
                HStack {
                    Text("LOG NOTE").font(FBase(style: .overline)).foregroundColor(Color("Text400"))
                    Spacer()
                }
                Text(content.userComment)
                HeaderBar(line1: "Your Signature", line2: "Scan it into your application")
                    .id("signqture QR")
                Image(uiImage: UIImage(data: Data(content.signature)) ?? UIImage())
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .padding(12)
                Spacer()
                BigButton(text: "Done", action: {
                    pushButton(.goBack, "", "")
                })
            }
            .onAppear{
                scrollView.scrollTo("signqture QR", anchor: .top)
            }
            .padding(16)
        }
    }
}

/*
 struct SignatureReady_Previews: PreviewProvider {
 static var previews: some View {
 SignatureReady()
 }
 }
 */
