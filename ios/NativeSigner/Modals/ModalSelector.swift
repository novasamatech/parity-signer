//
//  ModalSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.12.2021.
//

import SwiftUI

struct ModalSelector: View {
    @EnvironmentObject var data: SignerDataModel
    
    var body: some View {
        switch (data.actionResult.modal) {
        case .Empty:
            EmptyView()
        case .NewSeedMenu:
            NewSeedMenu()
        default:
            EmptyView()
        }
    }
}

/*
struct ModalSelector_Previews: PreviewProvider {
    static var previews: some View {
        ModalSelector()
    }
}
 */
