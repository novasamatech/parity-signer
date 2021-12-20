//
//  VerifierScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.12.2021.
//

import SwiftUI

struct VerifierScreen: View {
    @EnvironmentObject var data: SignerDataModel
    let content: MVerifierDetails
    var body: some View {
        HStack {
            Image(uiImage: UIImage(data: Data(fromHexEncodedString: content.identicon) ?? Data()) ?? UIImage())
            .resizable(resizingMode: .stretch)
            .frame(width: 42, height: 42)
            VStack{
                Text("General verifier certificate")
                Text(content.hex)
                Text("encryption: " + content.encryption)
            }
        }
    }
}


/*
 struct VerifierScreen_Previews: PreviewProvider {
 static var previews: some View {
 VerifierScreen()
 }
 }
 */
