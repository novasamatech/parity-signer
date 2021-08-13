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
        VStack {
            Text(history.timestamp)
            Text(String(history.events.count))
        }
        .foregroundColor(/*@START_MENU_TOKEN@*/Color("textMainColor")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct HistoryCollapsed_Previews: PreviewProvider {
    static var previews: some View {
        HistoryCollapsed()
    }
}
*/
