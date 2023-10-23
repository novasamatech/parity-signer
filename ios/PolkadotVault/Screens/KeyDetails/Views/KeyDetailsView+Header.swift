//
//  KeyDetailsView+Header.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 30/08/2023.
//

import SwiftUI

extension KeyDetailsView {
    @ViewBuilder
    func rootKeyHeader() -> some View {
        if let keySummary = viewModel.keySummary {
            VStack(alignment: .center, spacing: 0) {
                if let identicon = viewModel.keysData?.root?.address.identicon {
                    IdenticonView(
                        identicon: identicon,
                        rowHeight: Heights.identiconRootKeyDetails
                    )
                    .padding(.top, Spacing.extraSmall)
                }
                HStack(spacing: 0) {
                    Text(keySummary.keyName)
                        .font(PrimaryFont.titleXL.font)
                        .fixedSize(horizontal: false, vertical: true)
                        .multilineTextAlignment(.center)
                    Image(.chevronDown)
                        .resizable(resizingMode: .stretch)
                        .aspectRatio(contentMode: .fit)
                        .frame(width: Sizes.chevronDownKeyDetails)
                        .padding(Spacing.extraSmall)
                }
                .foregroundColor(.textAndIconsPrimary)
                .padding(.top, Spacing.medium)
                .padding(.bottom, Spacing.small)
                .contentShape(Rectangle())
                .onTapGesture { viewModel.onKeySetSelectionTap() }
                HStack {
                    Text(keySummary.base58.truncateMiddle())
                        .foregroundColor(.textAndIconsTertiary)
                        .font(PrimaryFont.bodyL.font)
                        .lineLimit(1)
                        .padding(.horizontal, Spacing.medium)
                        .padding(.vertical, Spacing.extraSmall)
                        .background(.fill6)
                        .clipShape(Capsule())
                }
                .contentShape(Rectangle())
                .onTapGesture { viewModel.onRootKeyTap() }
            }
            .padding(.horizontal, Spacing.large)
        } else {
            EmptyView()
        }
    }
}
