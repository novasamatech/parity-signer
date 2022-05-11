//
//  AlertSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.12.2021.
//

import SwiftUI
import CoreML

struct AlertSelector: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        switch (data.actionResult.alertData) {
        case .none:
            EmptyView()
        case .errorData(let value):
            ErrorAlert(content: value)
        case .shield(let value):
            ShieldAlertComponent(content: value ?? .past)
        case .confirm://(let value):
            let value = "TODO"
            ConfirmAlert(content: value)
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
