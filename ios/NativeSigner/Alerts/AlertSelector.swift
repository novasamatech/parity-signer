//
//  AlertSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.12.2021.
//

import SwiftUI

struct AlertSelector: View {
    @EnvironmentObject var data: SignerDataModel
    
    var body: some View {
        /*
        switch (data.actionResult.alertData) {
        case .Empty:
            EmptyView()
        case .Error(let value):
            ErrorAlert(content: value)
        case .Shield:
            ShieldAlert()
        case .Confirm(let value):
            ConfirmAlert(content: value)
        }
         */
        EmptyView()
    }
}

/*
struct AlertSelector_Previews: PreviewProvider {
    static var previews: some View {
        AlertSelector()
    }
}
*/
