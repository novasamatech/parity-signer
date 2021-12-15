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
            LazyVStack {
                ForEach(content.log.sorted(by: {$0.order > $1.order}), id: \.order) {history in
                    VStack {
                        ForEach(history.events, id: \.self) {event in
                            Button(action: {
                                
                            }) {
                                HistoryCard(
                                    event: event,
                                    timestamp: history.timestamp
                                        .padding(toLength: 16, withPad: " ", startingAt: 0))
                                        .foregroundColor(Color("Text400"))
                            }
                            .disabled(true)
                        }
                    }
                }
            }
        }
    }
}

/*
 struct HistoryScreen_Previews: PreviewProvider {
 static var previews: some View {
 HistoryScreen()
 }
 }*/
