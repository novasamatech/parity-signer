//
//  InfoBoxView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 28/11/2022.
//

import SwiftUI

struct InfoBoxView: View {
    let text: String

    var body: some View {
        HStack {
            Text(text)
                .frame(maxWidth: .infinity, alignment: .leading)
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            Spacer().frame(maxWidth: Spacing.medium)
            Asset.infoIconBold.swiftUIImage
                .foregroundColor(Asset.accentPink300.swiftUIColor)
        }
        .padding()
        .font(Fontstyle.bodyM.base)
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
