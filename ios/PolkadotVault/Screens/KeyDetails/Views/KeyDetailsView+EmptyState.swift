//
//  KeyDetailsView+EmptyState.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 30/08/2023.
//

import SwiftUI

extension KeyDetailsView {
    @ViewBuilder
    func emptyKeysList() -> some View {
        VStack(spacing: 0) {
            // Main key cell
            rootKeyHeader()
            // Derived Keys header
            listHeader()
            Spacer()
            // Empty state
            emptyState()
            Spacer()
        }
    }

    @ViewBuilder
    private func emptyState() -> some View {
        VStack(spacing: 0) {
            Localizable.KeyDetails.Label.EmptyState.header.text
                .font(PrimaryFont.titleM.font)
                .foregroundColor(.textAndIconsPrimary)
                .padding(.top, Spacing.large)
                .padding(.horizontal, Spacing.componentSpacer)
            ActionButton(
                action: viewModel.onCreateDerivedKeyTap,
                text: Localizable.KeyDetails.Label.EmptyState.action.key,
                style: .secondary()
            )
            .padding(Spacing.large)
        }
        .containerBackground(CornerRadius.large, state: .actionableInfo)
        .padding(.horizontal, Spacing.medium)
    }
}
