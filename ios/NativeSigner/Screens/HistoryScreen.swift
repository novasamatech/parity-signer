//
//  HistoryScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 4.10.2021.
//

import SwiftUI

struct HistoryScreen: View {
    @EnvironmentObject var data: SignerDataModel
    var content: MLog
    var body: some View {
        ScrollView {
            LazyVStack (spacing: 8) {
                ForEach(content.log.sorted(by: {$0.order > $1.order}), id: \.order) { history in
                    ForEach(history.events, id: \.self) { event in
                        Button(action: {
                            data.pushButton(buttonID: .ShowLogDetails, details: String(history.order))
                        }) {
                            HistoryCard(
                                event: event,
                                timestamp: history.timestamp.padding(toLength: 16, withPad: " ", startingAt: 0)
                            )
                            .foregroundColor(Color("Text400"))
                        }//.disabled(true)
                    }
                }
            }
            .padding(.horizontal, 8)
        }
    }
}

/*
 struct HistoryScreen_Previews: PreviewProvider {
 static var previews: some View {
 HistoryScreen()
 }
 }*/
