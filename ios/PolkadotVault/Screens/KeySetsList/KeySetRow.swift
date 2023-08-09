//
//  KeySetRow.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 22/08/2022.
//

import SwiftUI

struct KeySetRow: View {
    let viewModel: KeySetViewModel

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            if let derivedKeys = viewModel.derivedKeys {
                Text(derivedKeys)
                    .font(PrimaryFont.captionM.font)
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .padding(.bottom, Spacing.extraExtraSmall)
            }
            HStack(alignment: .center, spacing: 0) {
                Text(viewModel.keyName)
                    .font(PrimaryFont.titleL.font)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .fixedSize(horizontal: false, vertical: true)
                    .lineSpacing(Spacing.extraExtraSmall)
                    .padding(.trailing, Spacing.medium)
                Spacer()
                Asset.chevronRight.swiftUIImage
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .frame(height: Heights.chevronRightInList)
                    .foregroundColor(Asset.textAndIconsDisabled.swiftUIColor)
            }
            if !viewModel.networks.isEmpty {
                networksRow()
            }
        }
        .padding(Spacing.medium)
        .containerBackground(CornerRadius.small, state: .list)
    }

    @ViewBuilder
    func itemRow(_ item: String) -> some View {
        NetworkLogoIcon(
            networkName: item,
            size: Heights.networkLogoInList
        )
        .padding(1.5)
        .overlay(
            Circle()
                .stroke(Asset.backgroundSecondary.swiftUIColor, lineWidth: 4)
        )
    }

    private func itemsVisibility() -> (visible: Int, additional: Int) {
        let spaceContainer = UIScreen.main.bounds.width - Spacing.x3Large
        let itemWidth: CGFloat = Heights.networkLogoInList
        let itemSpacing: CGFloat = -5
        let itemTotalWidth = itemWidth + itemSpacing
        let itemsToFit = Int(floor(spaceContainer / itemTotalWidth)) - 2
        return (visible: itemsToFit, additional: viewModel.networks.count - itemsToFit)
    }

    @ViewBuilder
    private func networksRow() -> some View {
        HStack(alignment: .center, spacing: -5) {
            ForEach(
                0 ..< itemsVisibility().visible,
                id: \.self
            ) { itemIndex in
                if itemIndex < viewModel.networks.count {
                    itemRow(viewModel.networks[itemIndex])
                }
            }
            if itemsVisibility().additional > 0 {
                Text("+\(itemsVisibility().additional)")
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.labelS.font)
                    .frame(width: Heights.networkLogoInList, height: Heights.networkLogoInList)
                    .background(Asset.backgroundTertiary.swiftUIColor)
                    .clipShape(Circle())
                    .overlay(
                        Circle()
                            .stroke(Asset.fill12.swiftUIColor, lineWidth: 1)
                    )
            }
        }
        .padding(.top, Spacing.large)
    }
}

#if DEBUG
    struct KeySetRow_Previews: PreviewProvider {
        static var previews: some View {
            VStack(spacing: 0) {
                KeySetRow(
                    viewModel: .stub
                )
            }
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
        }
    }
#endif
