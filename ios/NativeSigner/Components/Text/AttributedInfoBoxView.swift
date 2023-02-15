//
//  AttributedInfoBoxView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 12/01/2022.
//

import SwiftUI

struct AttributedInfoBoxView: View {
    let text: AttributedString

    var body: some View {
        HStack {
            Text(text)
                .frame(maxWidth: .infinity, alignment: .leading)
            Spacer().frame(maxWidth: Spacing.medium)
            Asset.helpOutline.swiftUIImage
                .foregroundColor(Asset.accentPink300.swiftUIColor)
        }
        .padding()
        .font(PrimaryFont.bodyM.font)
        .background(
            RoundedRectangle(cornerRadius: CornerRadius.small)
                .stroke(Asset.fill12.swiftUIColor, lineWidth: 1)
                .cornerRadius(CornerRadius.small)
        )
    }
}

#if DEBUG
    struct AttributedInfoBoxView_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                AttributedInfoBoxView(text: Localizable.createDerivedKeyModalPathInfo())
                    .preferredColorScheme(.dark)
            }
        }
    }
#endif
