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
        ZStack {
            ScrollView(showsIndicators: false) {
                VStack(spacing: 0) {
                    // Main key cell
                    rootKeyHeader()
                    // Derived Keys header
                    listHeader()
                    // List
                    derivedKeys()
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
            }
            Spacer()
                .frame(height: Heights.actionButton + Spacing.large)
        }
    }
}
