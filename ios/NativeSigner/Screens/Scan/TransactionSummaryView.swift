//
//  TransactionSummaryView.swift
//  NativeSigner
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
            VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                Localizable.TransactionSign.Label.details.text
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(Fontstyle.captionM.base)
                HStack {
                    VStack(alignment: .leading) {
                        ForEach(renderable.summary.asRenderable, id: \.id) { row in
                            HStack(spacing: Spacing.extraSmall) {
                                Text(row.key)
                                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                                Text(row.value)
                                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                            }
                            .font(Fontstyle.captionM.base)
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
            Divider()
            VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                Localizable.TransactionSign.Label.sign.text
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(Fontstyle.captionM.base)
                HStack {
                    VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                        Text(renderable.signature.path)
                            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                            .font(Fontstyle.captionM.base)
                        Text(renderable.signature.name)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            .font(Fontstyle.bodyM.base)
                        HStack {
                            Text(
                                isShowingFullAddress ? renderable.signature.base58 : renderable.signature.base58
                                    .truncateMiddle()
                            )
                            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                            .font(Fontstyle.bodyM.base)

                            if !isShowingFullAddress {
                                Asset.chevronDown.swiftUIImage
                                    .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
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
                    Spacer()
                    Identicon(identicon: renderable.signature.identicon, rowHeight: Heights.identiconInCell)
                }
            }
        }
        .padding(Spacing.medium)
        .background(
            RoundedRectangle(cornerRadius: CornerRadius.small)
                .fill(Asset.fill6.swiftUIColor)
        )
    }
}

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
