//
//  HistoryScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 4.10.2021.
//

import SwiftUI

struct HistoryScreen: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        Text("History")
        /*
        ScrollView {
            LazyVStack {
                ForEach(data.history, id: \.order) {history in
                    VStack (alignment: .leading){
                        ForEach(history.events, id: \.self) {event in
                            Button(action: {
                                data.selectedRecord = history
                            }) {
                                HistoryCard(event: event, timestamp: history.timestamp.padding(toLength: 16, withPad: " ", startingAt: 0))
                                    .foregroundColor(/*@START_MENU_TOKEN@*/Color("textMainColor")/*@END_MENU_TOKEN@*/)
                                    .padding(.horizontal, 8)
                            }
                        }
                    }
                }
            }
        }
         */
    }
}

/*
 struct HistoryScreen_Previews: PreviewProvider {
 static var previews: some View {
 HistoryScreen()
 }
 }*/
