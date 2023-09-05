//
//  KeyDetailsView+MainList.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 30/08/2023.
//

import SwiftUI

extension KeyDetailsView {
    @ViewBuilder
    func derivedKeysList() -> some View {
        ScrollView(showsIndicators: false) {
            // Main key cell
            rootKeyHeader()
            // Derived Keys header
            HStack {
                Localizable.KeyDetails.Label.derived.text
                    .font(PrimaryFont.bodyM.font)
                Spacer().frame(maxWidth: .infinity)
                Asset.switches.swiftUIImage
                    .foregroundColor(
                        viewModel.isFilteringActive ? Asset.accentPink300.swiftUIColor : Asset
                            .textAndIconsTertiary.swiftUIColor
                    )
                    .frame(width: Heights.actionSheetButton)
                    .onTapGesture {
                        viewModel.onNetworkSelectionTap()
                    }
            }
            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            .padding(.horizontal, Spacing.large)
            .padding(.top, Spacing.medium)
            // List
            derivedKeys()
        }
    }

    @ViewBuilder
    private func derivedKeys() -> some View {
        LazyVStack(spacing: 0) {
            // List of derived keys
            ForEach(
                viewModel.derivedKeys,
                id: \.viewModel.addressKey
            ) { deriveKey in
                DerivedKeyRow(
                    viewModel: deriveKey.viewModel
                )
                .contentShape(Rectangle())
                .onTapGesture {
                    viewModel.onDerivedKeyTap(deriveKey)
                }
                NavigationLink(
                    destination:
                    KeyDetailsPublicKeyView(
                        viewModel: .init(
                            keyDetails: viewModel.presentedKeyDetails,
                            addressKey: viewModel.presentedPublicKeyDetails,
                            onCompletion: viewModel.onPublicKeyCompletion(_:)
                        )
                    )
                    .navigationBarHidden(true),
                    isActive: $viewModel.isPresentingKeyDetails
                ) { EmptyView() }
            }
            Spacer()
                .frame(height: Heights.actionButton + Spacing.large)
        }
    }
}
