//
//  SeedSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 27.7.2021.
//

import SwiftUI

struct SeedSelector: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        SeedCard(seedName: data.selectedSeed)
        .gesture(TapGesture()
                    .onEnded { _ in
            if data.getMultiSelectionMode() {
                if let rootAddress = data.getRootAddress(seedName: data.selectedSeed) {
                    data.multiSelectAction(address: rootAddress)
                }
            } else {
                data.multiSelected = []
                if let rootAddress = data.getRootAddress(seedName: data.selectedSeed) {
                    data.selectedAddress = rootAddress
                    data.keyManagerModal = .showKey
                }
            }
        })
        .gesture(LongPressGesture()
                    .onEnded { _ in
            if let rootAddress = data.getRootAddress(seedName: data.selectedSeed) {
                data.multiSelectAction(address: rootAddress)
            }
        })
    }
}

/*
 struct SeedSelector_Previews: PreviewProvider {
 static var previews: some View {
 SeedSelector().previewLayout(.sizeThatFits)
 }
 }
 */
