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
            VStack(spacing: 0) {
                // Main key cell
                rootKeyHeader()
                // Derived Keys header
                derivedKeysHeader()
                // List
                derivedKeys()
            }
        }
    }

    @ViewBuilder
    func emptyKeysList() -> some View {
        VStack(spacing: 0) {
            // Main key cell
            rootKeyHeader()
            // Derived Keys header
            derivedKeysHeader()
            Spacer()
            // Empty state
            emptyState()
            Spacer()
        }
    }

    @ViewBuilder
    func derivedKeysHeader() -> some View {
        HStack {
            Localizable.KeyDetails.Label.derived.text
                .font(PrimaryFont.bodyM.font)
            Spacer().frame(maxWidth: .infinity)
            Image(.switches)
                .foregroundColor(
                    viewModel.isFilteringActive ? .accentPink300 : .textAndIconsTertiary
                )
                .frame(width: Heights.actionSheetButton)
                .onTapGesture {
                    viewModel.onNetworkSelectionTap()
                }
        }
        .foregroundColor(.textAndIconsTertiary)
        .padding(.horizontal, Spacing.large)
        .padding(.top, Spacing.medium)
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
            Spacer()
                .frame(height: Heights.actionButton + Spacing.large)
        }
    }
}
