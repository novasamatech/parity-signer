//
//  EventDetails.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 28.10.2021.
//

import SwiftUI

struct EventDetails: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        VStack {
            /*
        Text(data.selectedRecord?.timestamp ?? "Timing error")
        ScrollView {
            LazyVStack {
                ForEach(data.selectedRecord?.events ?? [], id: \.self) {event in
                    HistoryCardExtended(event: event)
                        .foregroundColor(/*@START_MENU_TOKEN@*/Color("textMainColor")/*@END_MENU_TOKEN@*/)
                        .padding(.horizontal, 8)
                }
            }
             */
            Text("Details")
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
