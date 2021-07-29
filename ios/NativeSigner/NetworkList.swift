//
//  NetworkList.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI


struct NetworkList: View {
    @State private var networks = Networks()
    var body: some View {
        VStack {
            LazyVStack {
                ForEach(networks.data, id: \.key) {
                    network in
                    NavigationLink(destination: KeyList(network: network)) {
                        NetworkCard(network: network)
                    }
                }
                .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
            }
            .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
            Spacer()
            Footer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
        .navigationTitle("Select network").navigationBarTitleDisplayMode(.inline).toolbar {
            ToolbarItem(placement: .navigationBarTrailing) {
                NavbarShield()
            }
        }.background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
        
    }
}

struct NetworkList_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            NetworkList()
        }
        .foregroundColor(/*@START_MENU_TOKEN@*/.blue/*@END_MENU_TOKEN@*/)
        
    }
}
