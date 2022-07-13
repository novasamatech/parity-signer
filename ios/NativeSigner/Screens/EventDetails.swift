//
//  EventDetails.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 28.10.2021.
//

import SwiftUI

struct EventDetails: View {
    let content: MLogDetails
    var body: some View {
        VStack {
            Text(content.timestamp)
            ScrollView {
                LazyVStack {
                    ForEach(content.events, id: \.self) {event in
                        HistoryCardExtended(event: event)
                            .padding(.horizontal, 8)
                    }
                }
            }
        }
    }
}

/*
 struct EventDetails_Previews: PreviewProvider {
 static var previews: some View {
 EventDetails()
 }
 }
 */
