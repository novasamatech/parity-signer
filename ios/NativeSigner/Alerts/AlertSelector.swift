//
//  AlertSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.12.2021.
//

import SwiftUI
import CoreML

struct AlertSelector: View {
    let alertData: AlertData?

    var body: some View {
        switch alertData {
        case .none:
            EmptyView()
        case .errorData(let value):
            ErrorAlert(content: value)
        case .shield(let value):
            ShieldAlertComponent(content: value)
        case .confirm:// (let value):
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
