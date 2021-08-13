//
//  HistoryView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.8.2021.
//

import SwiftUI

struct HistoryView: View {
    @EnvironmentObject var data: SignerDataModel
    @Binding var showHistory: Bool
    @State var selectedRecord: Int?
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 50).foregroundColor(/*@START_MENU_TOKEN@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
            VStack {
                Text("History")
                    .font(.headline)
                    .foregroundColor(Color("AccentColor"))
                ScrollView {
                    LazyVStack {
                        ForEach(data.history, id: \.order) {history in
                            VStack {
                                if(selectedRecord == history.order) {
                                    Button(action: {
                                        selectedRecord = nil
                                    }) {
                                        HistoryExpanded(history: history)
                                    }
                                } else {
                                    Button(action: {
                                        selectedRecord = history.order
                                    }) {
                                        HistoryCollapsed(history: history)
                                    }
                                }
                            }
                        }
                    }
                }
                Spacer()
                Button(action: {showHistory = false}) {
                    Text("Back")
                        .font(.largeTitle)
                        .foregroundColor(Color("AccentColor"))
                }
            }
        }.padding(.bottom, 120)
    }
}

/*
struct HistoryView_Previews: PreviewProvider {
    static var previews: some View {
        HistoryView()
    }
}
*/
