//
//  AttributedTintInfoBox.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 08/02/2023.
//

import SwiftUI

struct AttributedTintInfoBox: View {
    let text: AttributedString

    var body: some View {
        HStack {
            Text(text)
                .frame(maxWidth: .infinity, alignment: .leading)
            Spacer().frame(width: Spacing.large)
            Asset.helpOutline.swiftUIImage
                .foregroundColor(Asset.accentPink300.swiftUIColor)
        }
        .padding(Spacing.medium)
        .font(PrimaryFont.bodyM.font)
        .background(
            RoundedRectangle(cornerRadius: CornerRadius.medium)
                .foregroundColor(Asset.accentPink300Fill8.swiftUIColor)
        )
    }
}

#if DEBUG
    struct AttributedTintInfoBox_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                AttributedTintInfoBox(text: Localizable.createKeySetSeedPhraseInfo())
                    .preferredColorScheme(.dark)
            }
        }
    }
#endif
