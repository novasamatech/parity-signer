//
//  HistoryScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 4.10.2021.
//

import SwiftUI

struct HistoryScreen: View {
    @EnvironmentObject private var data: SignerDataModel
    let content: MLog
    let navigationRequest: NavigationRequest
    var body: some View {
        ScrollView {
            LazyVStack(spacing: 8) {
                ForEach(content.log, id: \.timestamp) { history in
                    ForEach(history.events, id: \.self) { event in
                        Button(
                            action: {
                                navigationRequest(
                                    .init(
                                        action: .showLogDetails,
                                        details: String(content.log.reversed().firstIndex(of: history) ?? 0)
                                    )
                                )
                            },
                            label: {
                                HistoryCard(
                                    event: event,
                                    timestamp: history.timestamp.padding(toLength: 16, withPad: " ", startingAt: 0)
                                )
                                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                            }
                        )
                    }
                }
            }
            .padding(.horizontal, 8)
        }.padding(.bottom, -20)
    }
}

// struct HistoryScreen_Previews: PreviewProvider {
// static var previews: some View {
// HistoryScreen()
// }
// }
