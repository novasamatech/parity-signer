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
            Image(.airplane)
            Text("Use Signer in Airplane mode").font(Fontstyle.body2.base).foregroundColor(Asset.text600.swiftUIColor)
            Text(
                "Airplane mode will stop your phone from using mobile data." +
                    " Signer will only work when you have no wifi and no mobile connection!"
            )
            .font(Fontstyle.subtitle2.base).foregroundColor(Asset.text300.swiftUIColor)
            Image(.wifi, variant: .slash)
            Text("Airgap your phone").font(Fontstyle.body2.base).foregroundColor(Asset.text600.swiftUIColor)
            Text(
                "Make sure your phone's Bluetooth, NFC and other sensors are off," +
                    " and that all cables are disconnected." +
                    " Signer will not check these connections, so it is important that you do!"
            )
            .font(Fontstyle.subtitle2.base).foregroundColor(Asset.text300.swiftUIColor)
        }.padding(16).background(RoundedRectangle(cornerRadius: 8).foregroundColor(Asset.bg200.swiftUIColor))
    }
}

// struct InstructionsSquare_Previews: PreviewProvider {
//    static var previews: some View {
//        InstructionsSquare()
//    }
// }
