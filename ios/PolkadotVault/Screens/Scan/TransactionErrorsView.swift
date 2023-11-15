//
//  TransactionErrorsView.swift
//  Polkadot Vault
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
            .font(PrimaryFont.bodyM.font)
            .foregroundColor(.accentRed300)
            .strokeContainerBackground(CornerRadius.small, state: .error)
        } else {
            EmptyView()
        }
    }
}
