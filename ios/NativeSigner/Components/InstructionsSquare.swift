//
//  InstructionsSquare.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.1.2022.
//

import SwiftUI

struct InstructionsSquare: View {
    var body: some View {
        VStack(alignment: .leading) {
            Image(systemName: "airplane")
            Text("Use Signer in Airplane mode").font(Fontstyle.body2.base).foregroundColor(Color("Text600"))
            Text(
                "Airplane mode will stop your phone from using mobile data." +
                    " Signer will only work when you have no wifi and no mobile connection!"
            )
            .font(Fontstyle.subtitle2.base).foregroundColor(Color("Text300"))
            Image(systemName: "wifi.slash")
            Text("Airgap your phone").font(Fontstyle.body2.base).foregroundColor(Color("Text600"))
            Text(
                "Make sure your phone's Bluetooth, NFC and other sensors are off," +
                    " and that all cables are disconnected." +
                    " Signer will not check these connections, so it is important that you do!"
            )
            .font(Fontstyle.subtitle2.base).foregroundColor(Color("Text300"))
        }.padding(16).background(RoundedRectangle(cornerRadius: 8).foregroundColor(Color("Bg200")))
    }
}

// struct InstructionsSquare_Previews: PreviewProvider {
//    static var previews: some View {
//        InstructionsSquare()
//    }
// }
