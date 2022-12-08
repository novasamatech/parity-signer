//
//  TransactionErrorsView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 30/11/2022.
//

import SwiftUI

struct TransactionErrorsView: View {
    let content: MTransaction

    var body: some View {
        if !content.transactionIssuesCards().isEmpty {
            VStack {
                HStack {
                    Text(content.transactionIssues())
                    Spacer()
                }
            }
            .padding(Spacing.medium)
            .font(Fontstyle.bodyM.base)
            .foregroundColor(Asset.accentRed300.swiftUIColor)
            .strokeContainerBackground(CornerRadius.small, isError: true)
        } else {
            EmptyView()
        }
    }
}
