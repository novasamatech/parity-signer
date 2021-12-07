//
//  AlertSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.12.2021.
//

import SwiftUI

struct AlertSelector: View {
    @EnvironmentObject var data: SignerDataModel
    
    var body: some View {
        switch (data.actionResult.alert) {
        default:
            EmptyView()
        }
    }
}

/*
struct AlertSelector_Previews: PreviewProvider {
    static var previews: some View {
        AlertSelector()
    }
}
*/
