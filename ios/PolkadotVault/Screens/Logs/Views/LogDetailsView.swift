//
//  LogDetailsView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 20/03/2023.
//

import SwiftUI

struct LogDetailsView: View {
    @StateObject var viewModel: ViewModel
    @Environment(\.presentationMode) var mode: Binding<PresentationMode>

    var body: some View {
        VStack(alignment: .center, spacing: 0) {
            NavigationBarView(
                viewModel: .init(
                    title: Localizable.LogDetails.Label.title.string,
                    leftButtons: [.init(type: .arrow, action: { mode.wrappedValue.dismiss() })]
                )
            )
            Text(viewModel.details.timestamp)
                .padding(.bottom, Spacing.small)
            ScrollView {
                LazyVStack {
                    ForEach(viewModel.details.events, id: \.self) { event in
                        HistoryCardExtended(event: event)
                            .padding(.horizontal, Spacing.extraSmall)
                            .padding(.bottom, Spacing.extraSmall)
                    }
                }
            }
        }
    }
}

extension LogDetailsView {
    final class ViewModel: ObservableObject {
        let details: MLogDetails

        init(_ details: MLogDetails) {
            self.details = details
        }
    }
}
