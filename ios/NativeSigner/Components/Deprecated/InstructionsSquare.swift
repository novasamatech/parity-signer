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
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
            Localizable.AirplaneMode.explanation.text
                .font(PrimaryFont.bodyM.font)
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            Image(.wifi, variant: .slash)
            Localizable.airgapYourPhone.text
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
            Localizable.Connectivity.explanation.text
                .font(PrimaryFont.bodyM.font).foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
        }
        .padding(16)
        .background(
            RoundedRectangle(cornerRadius: 8)
                .foregroundColor(Asset.backgroundSecondary.swiftUIColor)
        )
    }
}

// struct InstructionsSquare_Previews: PreviewProvider {
//    static var previews: some View {
//        InstructionsSquare()
//    }
// }
