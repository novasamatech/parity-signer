//
//  KeySetRow.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 22/08/2022.
//

import SwiftUI

struct KeySetRow: View {
    private let viewModel: KeySetViewModel

    init(_ viewModel: KeySetViewModel) {
        self.viewModel = viewModel
    }

    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: CornerRadius.small)
                .foregroundColor(Asset.backgroundPrimary.swiftUIColor)
                .frame(height: Heights.keyCellContainer)
            HStack(alignment: .center, spacing: Spacing.small) {
                Identicon(identicon: viewModel.identicon, rowHeight: Heights.identiconInCell)
                VStack(alignment: .leading) {
                    Text(viewModel.keyName)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(Fontstyle.titleS.base)
                    if let derivedKeys = viewModel.derivedKeys {
                        Spacer().frame(height: 2)
                        Text(derivedKeys)
                            .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                            .font(Fontstyle.bodyM.base)
                    }
                }
                Spacer()
                Asset.chevronRight.swiftUIImage
                    .foregroundColor(Asset.textAndIconsDisabled.swiftUIColor)
            }
            .padding(Spacing.medium)
        }
        .background(Asset.backgroundSystem.swiftUIColor)
    }
}

struct KeySetRow_Previews: PreviewProvider {
    static var previews: some View {
        VStack {
            KeySetRow(
                KeySetViewModel(
                    keyName: "Parity",
                    derivedKeys: "1 Derived Key",
                    identicon: PreviewData.exampleIdenticon
                )
            )
            KeySetRow(
                KeySetViewModel(
                    keyName: "Kusama",
                    derivedKeys: nil,
                    identicon: PreviewData.exampleIdenticon
                )
            )
            KeySetRow(
                KeySetViewModel(
                    keyName: "Dotsama crowdloans",
                    derivedKeys: "3 Derived Keys",
                    identicon: PreviewData.exampleIdenticon
                )
            )
        }
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
    }
}
