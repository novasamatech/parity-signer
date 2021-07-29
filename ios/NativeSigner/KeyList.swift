//
//  KeyList.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

struct KeyList: View {
    @Environment(\.presentationMode) var presentationMode: Binding<PresentationMode>
    @State var selectedSeed = ""
    var seeds = ["Alice", "Bob"]
    var network: Network
    var body: some View {
        VStack {
            Button(action: {presentationMode.wrappedValue.dismiss()}) {
                NetworkCard(network: network)
            }
            SeedSelector(selectedSeed: $selectedSeed, seeds: seeds)
/*            LazyVStack {
                ForEach(networks.data, id: \.key) {
                    network in
                    NavigationLink(destination: Text(network.key)) {
                        NetworkCard(network: network)
                    }
                }
            }*/
            Spacer()
            Footer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
        .navigationTitle("Manage identities").navigationBarTitleDisplayMode(.inline).toolbar {
            ToolbarItem(placement: .navigationBarTrailing) {
                NavbarShield()
            }
        }.background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
    }
}

struct KeyList_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            KeyList(network: Network.networkData[0])
        }
    }
}
