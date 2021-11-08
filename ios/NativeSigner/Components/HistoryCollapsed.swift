//
//  HistoryCollapsed.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.8.2021.
//

import SwiftUI

struct HistoryCollapsed: View {
    var history: History
    var body: some View {
        VStack(alignment: .leading) {
            ForEach(history.events, id: \.self) {event in
                HistoryCard(event: event, timestamp: history.timestamp.padding(toLength: 16, withPad: " ", startingAt: 0))
            }
        }
        .foregroundColor(/*@START_MENU_TOKEN@*/Color("textMainColor")/*@END_MENU_TOKEN@*/)
        .padding(.horizontal, 8)
    }
}

/*
 struct HistoryCollapsed_Previews: PreviewProvider {
 static var previews: some View {
 HistoryCollapsed()
 }
 }
 */
