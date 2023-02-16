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
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.captionM.font)
                    .padding(.bottom, Spacing.extraSmall)
                HStack {
                    VStack(alignment: .leading, spacing: 0) {
                        ForEach(renderable.summary.asRenderable, id: \.id) { row in
                            HStack(spacing: Spacing.extraSmall) {
                                Text(row.key)
                                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                                Text(row.value)
                                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            }
                            .font(PrimaryFont.bodyM.font)
                            .frame(minHeight: Heights.minTransactionSummaryItemHeight)
                        }
                    }
                    Spacer()
                    Asset.chevronRight.swiftUIImage
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                        .padding(Spacing.extraSmall)
                }
            }
            .contentShape(Rectangle())
            .onTapGesture { onTransactionDetailsTap() }
            signature()
        }
        .padding(Spacing.medium)
        .background(
            RoundedRectangle(cornerRadius: CornerRadius.small)
                .fill(Asset.fill6.swiftUIColor)
        )
    }

    @ViewBuilder
    func signature() -> some View {
        if let signature = renderable.signature {
            Divider()
            VStack(alignment: .leading, spacing: 0) {
                Localizable.TransactionSign.Label.sign.text
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.captionM.font)
                    .padding(.bottom, Spacing.extraSmall)
                HStack {
                    signatureDetails(signature)
                    Spacer()
                    NetworkIdenticon(
                        identicon: signature.identicon,
                        network: signature.network,
                        background: Asset.fill6Solid.swiftUIColor,
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
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                .font(PrimaryFont.captionM.font)
            Text(signature.name)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.bodyM.font)
            HStack {
                Text(
                    isShowingFullAddress ? signature.base58 : signature.base58
                        .truncateMiddle()
                )
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                .font(PrimaryFont.bodyM.font)

                if !isShowingFullAddress {
                    Asset.chevronDown.swiftUIImage
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
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
                    summary: PreviewData.transactionSummary,
                    signature: PreviewData.transactionSignature
                ),
                onTransactionDetailsTap: {}
            )
            .preferredColorScheme(.dark)
        }
    }
#endif
