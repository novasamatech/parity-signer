//
//  TransactionDetailsView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 20/11/2022.
//

import SwiftUI

struct TransactionDetailsView: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: .init(
                    title: .title(Localizable.TransactionPreview.Label.title.string),
                    leftButtons: [.init(type: .xmark, action: viewModel.onBackButtonTap)],
                    rightButtons: [.init(type: .empty)]
                )
            )
            ScrollView {
                VStack(spacing: 0) {
                    TransactionErrorsView(content: viewModel.transaction)
                        .padding(.bottom, Spacing.medium)
                    ForEach(viewModel.transaction.sortedValueCards(), id: \.index) { card in
                        TransactionCardView(card: card)
                    }
                    Spacer()
                        .frame(height: Spacing.extraExtraLarge)
                }
                .padding(.horizontal, Spacing.large)
            }
        }
        .background(.backgroundPrimary)
    }
}

extension TransactionDetailsView {
    final class ViewModel: ObservableObject {
        @Binding var isPresented: Bool

        let transaction: MTransaction

        init(
            isPresented: Binding<Bool>,
            transaction: MTransaction
        ) {
            _isPresented = isPresented
            self.transaction = transaction
        }

        func onBackButtonTap() {
            isPresented = false
        }
    }
}
