//
//  TransparentHelpBox.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 23/06/2023.
//

import SwiftUI

struct TransparentHelpBox: View {
    let text: String

    var body: some View {
        HStack {
            Text(text)
                .frame(maxWidth: .infinity, alignment: .leading)
                .fixedSize(horizontal: false, vertical: true)
                .foregroundColor(.textAndIconsTertiary)
            Spacer().frame(maxWidth: Spacing.medium)
            Image(.helpOutline)
                .foregroundColor(.accentPink300)
        }
        .padding(Spacing.medium)
        .font(PrimaryFont.bodyM.font)
        .background(
            RoundedRectangle(cornerRadius: CornerRadius.small)
                .stroke(.fill12, lineWidth: 1)
                .cornerRadius(CornerRadius.small)
        )
    }
}

#if DEBUG
    struct TransparentHelpBox_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                TransparentHelpBox(text: Localizable.KeysExport.KeySets.Label.info.string)
                    .preferredColorScheme(.dark)
            }
            VStack {
                TransparentHelpBox(text: Localizable.KeysExport.KeySets.Label.info.string)
                    .preferredColorScheme(.light)
            }
        }
    }
#endif
