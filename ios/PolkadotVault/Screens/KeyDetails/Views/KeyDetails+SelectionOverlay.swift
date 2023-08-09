//
//  KeyDetails+SelectionOverlay.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 30/10/2022.
//

import SwiftUI

extension KeyDetailsView {
    var selectKeysOverlay: some View {
        VStack {
            // Top overlay
            HStack {
                NavbarButton(
                    action: viewModel.toggleSelectKeysOverlay,
                    icon: Image(.xmark)
                )
                .padding(.leading, Spacing.extraSmall)
                Spacer()
                Text(selectionTitle)
                    .font(PrimaryFont.titleS.font)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor).lineLimit(1)
                Spacer()
                Spacer().frame(width: Heights.navigationButton)
            }
            .frame(height: Heights.tabbarHeight)
            .background(Asset.backgroundSecondary.swiftUIColor)
            Spacer()
            // Bottom overlay
            HStack {
                // Select All
                Button(action: { selectAll() }) {
                    Localizable.KeyDetails.Overlay.Action.selectAll.text
                        .foregroundColor(Asset.accentPink300.swiftUIColor)
                        .font(PrimaryFont.labelL.font)
                }
                .padding(.leading, Spacing.medium)
                Spacer()
                // Export
                Button(action: viewModel.exportSelectedKeys) {
                    Localizable.KeyDetails.Overlay.Action.export.text
                        .foregroundColor(Asset.accentPink300.swiftUIColor)
                        .font(PrimaryFont.labelL.font)
                }
                .padding(.trailing, Spacing.medium)
            }
            .frame(height: Heights.tabbarHeight)
            .background(Asset.backgroundSecondary.swiftUIColor)
        }
    }

    var selectionTitle: String {
        let localizable = Localizable.KeyDetails.Overlay.Label.self
        let itemsCount = viewModel.selectedKeys.count
        let result: String
        switch itemsCount {
        case 0:
            result = localizable.Title.empty.string
        case 1:
            result = localizable.title(String(itemsCount), localizable.Key.single.string)
        default:
            result = localizable.title(String(itemsCount), localizable.Key.plural.string)
        }
        return result
    }

    func selectAll() {
        viewModel.selectedKeys = viewModel.derivedKeys
    }
}
