//
//  InfoBoxView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 28/11/2022.
//

import SwiftUI

struct InfoBoxView: View {
    let text: String

    var body: some View {
        HStack {
            Text(text)
                .lineLimit(nil)
                .multilineTextAlignment(.leading)
                .fixedSize(horizontal: false, vertical: true)
                .foregroundColor(.textAndIconsTertiary)
            Spacer().frame(maxWidth: Spacing.medium)
            Image(.infoIconBold)
                .foregroundColor(.accentPink300)
        }
        .padding()
        .frame(maxWidth: .infinity)
        .font(PrimaryFont.bodyM.font)
        .strokeContainerBackground(CornerRadius.small)
    }
}

#if DEBUG
    struct InfoBoxView_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                InfoBoxView(text: Localizable.KeysExport.KeySets.Label.info.string)
                    .preferredColorScheme(.dark)
            }
            VStack {
                InfoBoxView(text: Localizable.KeysExport.KeySets.Label.info.string)
                    .preferredColorScheme(.light)
            }
        }
    }
#endif
