//
//  TransactionSummaryView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 21/11/2022.
//

import SwiftUI

struct TransactionSummaryView: View {
    var renderable: TransactionPreviewRenderable
    let onTransactionDetailsTap: () -> Void
    @State var isShowingFullAddress: Bool = false

    var body: some View {
        VStack(alignment: .leading, spacing: Spacing.extraSmall) {
            VStack(alignment: .leading, spacing: 0) {
                Localizable.TransactionSign.Label.details.text
                    .foregroundColor(.textAndIconsTertiary)
                    .font(PrimaryFont.captionM.font)
                    .padding(.bottom, Spacing.extraSmall)
                HStack {
                    VStack(alignment: .leading, spacing: 0) {
                        ForEach(renderable.summary.asRenderable, id: \.id) { row in
                            HStack(spacing: Spacing.extraSmall) {
                                Text(row.key)
                                    .foregroundColor(.textAndIconsTertiary)
                                Text(row.value)
                                    .foregroundColor(.textAndIconsPrimary)
                            }
                            .font(PrimaryFont.bodyM.font)
                            .frame(minHeight: Heights.minTransactionSummaryItemHeight)
                        }
                    }
                    Spacer()
                    Image(.chevronRight)
                        .foregroundColor(.textAndIconsTertiary)
                        .padding(Spacing.extraSmall)
                }
            }
            .contentShape(Rectangle())
            .onTapGesture { onTransactionDetailsTap() }
            signature()
        }
        .padding(Spacing.medium)
        .containerBackground(CornerRadius.small, state: .standard)
    }

    @ViewBuilder
    func signature() -> some View {
        if let signature = renderable.signature {
            Divider()
            VStack(alignment: .leading, spacing: 0) {
                Localizable.TransactionSign.Label.sign.text
                    .foregroundColor(.textAndIconsTertiary)
                    .font(PrimaryFont.captionM.font)
                    .padding(.bottom, Spacing.extraSmall)
                HStack {
                    signatureDetails(signature)
                    Spacer()
                    NetworkIdenticon(
                        identicon: signature.identicon,
                        network: signature.network,
                        background: .fill6Solid,
                        size: Heights.identiconInCell
                    )
                }
            }
        } else {
            EmptyView()
        }
    }

    @ViewBuilder
    private func signatureDetails(_ signature: TransactionSignatureRenderable) -> some View {
        VStack(alignment: .leading, spacing: Spacing.minimal) {
            renderablePath(for: signature)
                .foregroundColor(.textAndIconsTertiary)
                .font(PrimaryFont.captionM.font)
            Text(signature.name)
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.bodyM.font)
            HStack {
                Text(
                    isShowingFullAddress ? signature.base58 : signature.base58
                        .truncateMiddle()
                )
                .foregroundColor(.textAndIconsTertiary)
                .font(PrimaryFont.bodyM.font)

                if !isShowingFullAddress {
                    Image(.chevronDown)
                        .foregroundColor(.textAndIconsTertiary)
                        .padding(.leading, Spacing.extraExtraSmall)
                }
            }
            .contentShape(Rectangle())
            .onTapGesture {
                withAnimation {
                    isShowingFullAddress = true
                }
            }
        }
    }

    /// Manual string interpolation for `lock` `SFSymbol`
    private func renderablePath(for signature: TransactionSignatureRenderable) -> Text {
        signature.hasPassword ?
            Text("\(signature.path)\(Image(.lock))") :
            Text(signature.path)
    }
}

#if DEBUG
    struct TransactionSummaryView_Previews: PreviewProvider {
        static var previews: some View {
            TransactionSummaryView(
                renderable: .init(
                    summary: .stub,
                    signature: .stub
                ),
                onTransactionDetailsTap: {}
            )
            .preferredColorScheme(.dark)
        }
    }
#endif
