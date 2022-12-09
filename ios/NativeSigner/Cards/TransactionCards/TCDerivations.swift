//
//  TCDerivations.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 6.1.2022.
//

import SwiftUI

struct TCDerivations: View {
    let value: [String]
    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                Localizable.importingDerivations.text
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.bodyL.font)
                ForEach(value, id: \.self) { derivation in
                    HStack {
                        Text(derivation)
                            .font(PrimaryFont.bodyM.font)
                            .foregroundColor(Asset.accentPink300.swiftUIColor)
                        Spacer()
                    }
                }
            }
        }
    }
}

struct TCDerivations_Previews: PreviewProvider {
    static var previews: some View {
        TCDerivations(value: ["Derivation 1", "Derivation 2"])
    }
}
