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
                .foregroundColor(Asset.fill6.swiftUIColor)
                .frame(height: 72)
            HStack(alignment: .center, spacing: Padding.small) {
                Identicon(identicon: viewModel.identicon, rowHeight: 36)
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
            .padding(Padding.medium)
        }
    }
}

struct AddressCardSelector_Previews: PreviewProvider {
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
