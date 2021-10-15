//
//  NetworkList.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI


struct NetworkSelector: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        Button(action: {
            data.multiSelected = []
            if data.keyManagerModal == .networkManager {
                data.keyManagerModal = .none
            } else {
                data.keyManagerModal = .networkManager
            }
        })  {
            HStack {
                NetworkCard(network: data.selectedNetwork).padding()
                Image(systemName: data.keyManagerModal == .networkManager ? "chevron.up.circle" : "chevron.down.circle").imageScale(.large).foregroundColor(Color("AccentColor"))
                Spacer()
            }
        }
    }
}

/*
 struct NetworkList_Previews: PreviewProvider {
 static var previews: some View {
 NetworkList().previewLayout(.sizeThatFits)
 }
 }
 */
