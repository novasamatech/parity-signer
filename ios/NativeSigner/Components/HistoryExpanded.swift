//
//  HistoryExpanded.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.8.2021.
//

import SwiftUI

struct HistoryExpanded: View {
    var history: History
    var body: some View {
        VStack {
            Text(history.timestamp)
            Text(String(history.events.count))
            VStack {
                ForEach(history.events, id: \.self) {event in
                    Text(String(reflecting: event))
                }
            }
        }
        .foregroundColor(/*@START_MENU_TOKEN@*/Color("textMainColor")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct HistoryExpanded_Previews: PreviewProvider {
    static var previews: some View {
        HistoryExpanded()
    }
}
*/
