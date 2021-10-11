//
//  HistoryScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 4.10.2021.
//

import SwiftUI

struct HistoryScreen: View {
    @EnvironmentObject var data: SignerDataModel
    @State var selectedRecord: Event?
    var body: some View {
        if (selectedRecord == nil) {
            ScrollView {
                LazyVStack {
                    ForEach(data.history, id: \.order) {history in
                        VStack (alignment: .leading){
                            ForEach(history.events, id: \.self) {event in
                                Button(action: {
                                    selectedRecord = event
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
            .onAppear {
                data.getHistory()
            }
        } else {
            //TODO
            ZStack {
                RoundedRectangle(cornerRadius: 50).foregroundColor(/*@START_MENU_TOKEN@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                VStack {
                    Text("This is not implemented yet, sorry!")
                        .font(.headline).foregroundColor(Color("dangerColor"))
                    Text(String(describing: selectedRecord!))
                    Button(action: {selectedRecord = nil}) {
                        Text("Back")
                            .font(.largeTitle)
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
