//
//  KeyDetails+SelectionOverlay.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 30/10/2022.
//

import SwiftUI

extension KeyDetailsView {
    var selectKeysOverlay: some View {
        VStack {
            // Top overlay
            HStack {
                NavbarButton(action: { viewModel.isPresentingSelectionOverlay.toggle() }, icon: Image(.xmark))
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
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.labelL.font)
                }
                .padding(.leading, Spacing.medium)
                Spacer()
                // Export
                Button(action: { viewModel.isShowingKeysExportModal.toggle() }) {
                    Localizable.KeyDetails.Overlay.Action.export.text
                        .foregroundColor(
                            viewModel.selectedSeeds.isEmpty ? Asset.textAndIconsDisabled.swiftUIColor : Asset
                                .textAndIconsPrimary.swiftUIColor
                        )
                        .font(PrimaryFont.labelL.font)
                }
                .padding(.trailing, Spacing.medium)
                .disabled(viewModel.selectedSeeds.isEmpty)
            }
            .frame(height: Heights.tabbarHeight)
            .background(Asset.backgroundSecondary.swiftUIColor)
        }
    }

    var selectionTitle: String {
        let localizable = Localizable.KeyDetails.Overlay.Label.self
        let itemsCount = viewModel.selectedSeeds.count
        let keyString = itemsCount == 1 ? localizable.Key.single.string : localizable.Key.plural.string
        return localizable.title(String(itemsCount), keyString)
    }

    func selectAll() {
        viewModel.selectedSeeds = dataModel.derivedKeys.map(\.viewModel.path)
    }
}
