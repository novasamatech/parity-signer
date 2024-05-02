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
                .lineLimit(nil)
                .multilineTextAlignment(.leading)
                .fixedSize(horizontal: false, vertical: true)
            Spacer().frame(maxWidth: Spacing.medium)
            Image(.helpOutline)
                .foregroundColor(.accentPink300)
        }
        .padding()
        .frame(maxWidth: .infinity)
        .font(PrimaryFont.bodyM.font)
        .background(
            RoundedRectangle(cornerRadius: CornerRadius.small)
                .stroke(.fill12, lineWidth: 1)
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
