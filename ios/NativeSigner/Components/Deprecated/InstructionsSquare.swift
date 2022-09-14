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
            Localizable.useSignerInAirplaneMode.text
                .font(Fontstyle.body2.base)
                .foregroundColor(Asset.text600.swiftUIColor)
            Localizable.AirplaneMode.explanation.text
                .font(Fontstyle.subtitle2.base)
                .foregroundColor(Asset.text300.swiftUIColor)
            Image(.wifi, variant: .slash)
            Localizable.airgapYourPhone.text
                .font(Fontstyle.body2.base)
                .foregroundColor(Asset.text600.swiftUIColor)
            Localizable.Connectivity.explanation.text
                .font(Fontstyle.subtitle2.base).foregroundColor(Asset.text300.swiftUIColor)
        }
        .padding(16)
        .background(
            RoundedRectangle(cornerRadius: 8)
                .foregroundColor(Asset.bg200.swiftUIColor)
        )
    }
}

// struct InstructionsSquare_Previews: PreviewProvider {
//    static var previews: some View {
//        InstructionsSquare()
//    }
// }
