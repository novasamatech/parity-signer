//
//  KeySetRow.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 22/08/2022.
//

import SwiftUI

struct KeySetRow: View {
    let viewModel: KeySetViewModel
    @Binding var selectedItems: [KeySetViewModel]
    @Binding var isExportKeysSelected: Bool

    private var isItemSelected: Bool {
        selectedItems.contains(viewModel)
    }

    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: CornerRadius.small)
                .foregroundColor(Asset.backgroundSecondary.swiftUIColor)
                .frame(height: Heights.keyCellContainer)
            HStack(alignment: .center, spacing: Spacing.small) {
                Identicon(identicon: viewModel.identicon, rowHeight: Heights.identiconInCell)
                VStack(alignment: .leading) {
                    Text(viewModel.keyName)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.titleS.font)
                    if let derivedKeys = viewModel.derivedKeys {
                        Spacer().frame(height: 2)
                        Text(derivedKeys)
                            .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                            .font(PrimaryFont.bodyM.font)
                    }
                }
                Spacer()
                if isExportKeysSelected {
                    isItemSelected ? Asset.checkmarkChecked.swiftUIImage : Asset.checkmarkUnchecked.swiftUIImage
                } else {
                    Asset.chevronRight.swiftUIImage
                        .foregroundColor(Asset.textAndIconsDisabled.swiftUIColor)
                }
            }
            .padding(Spacing.medium)
        }
    }
}

#if DEBUG
    struct KeySetRow_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                KeySetRow(
                    viewModel: KeySetViewModel(
                        seed: PreviewData.seedNameCard,
                        keyName: "Main Polkadot",
                        derivedKeys: "1 Derived Key",
                        identicon: PreviewData.exampleIdenticon
                    ),
                    selectedItems: Binding<[KeySetViewModel]>.constant([]),
                    isExportKeysSelected: Binding<Bool>.constant(true)
                )
                KeySetRow(
                    viewModel: KeySetViewModel(
                        seed: PreviewData.seedNameCard,
                        keyName: "Kusama",
                        derivedKeys: nil,
                        identicon: PreviewData.exampleIdenticon
                    ),
                    selectedItems: Binding<[KeySetViewModel]>.constant([]),
                    isExportKeysSelected: Binding<Bool>.constant(false)
                )
                KeySetRow(
                    viewModel: KeySetViewModel(
                        seed: PreviewData.seedNameCard,
                        keyName: "Dotsama crowdloans",
                        derivedKeys: "3 Derived Keys",
                        identicon: PreviewData.exampleIdenticon
                    ),
                    selectedItems: Binding<[KeySetViewModel]>.constant([KeySetViewModel(
                        seed: PreviewData.seedNameCard,
                        keyName: "Dotsama crowdloans",
                        derivedKeys: "3 Derived Keys",
                        identicon: PreviewData.exampleIdenticon
                    )]),
                    isExportKeysSelected: Binding<Bool>.constant(true)
                )
            }
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
        }
    }
#endif
