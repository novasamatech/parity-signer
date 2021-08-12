//
//  SeedManager.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.8.2021.
//

import SwiftUI

struct SeedManager: View {
    @EnvironmentObject var data: SignerDataModel
    @Binding var showSeedManager: Bool
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 50).foregroundColor(/*@START_MENU_TOKEN@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
            VStack {
                Text("Seeds")
                    .font(.largeTitle)
                    .foregroundColor(Color("AccentColor"))
            ScrollView {
                LazyVStack {
                    ForEach(data.seedNames, id: \.self) {seed in
                        VStack {
                            if(seed == data.selectedSeed) {
                                Button(action: {
                                        data.selectedSeed = ""
                                }) {
                                    Text(seed)
                                        .font(.largeTitle)
                                        .foregroundColor(Color("AccentColor"))
                                }
                                HStack {
                                    Button(action:{}) {
                                        Text("Delete")
                                            .font(.largeTitle)
                                            .foregroundColor(Color("AccentColor"))
                                    }
                                    Spacer()
                                    Button(action:{}) {
                                        Text("Backup")
                                            .font(.largeTitle)
                                            .foregroundColor(Color("AccentColor"))
                                    }
                                }
                            } else {
                                Button(action: {
                                        data.selectSeed(seedName: seed)
                                }) {
                                    Text(seed)
                                        .font(.largeTitle)
                                        .foregroundColor(Color("AccentColor"))
                                }
                            }
                        }
                    }
                }
            }
            }
            Spacer()
            Button(action: {showSeedManager = false})
                {
                Text("Back")
                    .font(.largeTitle)
                    .foregroundColor(Color("AccentColor"))
            }
        }.padding(.bottom, 100)
    }
}

/*
struct SeedManager_Previews: PreviewProvider {
    static var previews: some View {
        SeedManager()
    }
}
*/
