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
    let canaryDead: Bool
    let resetAlert: () -> Void
    let pushButton: (Action, String, String) -> Void

    var body: some View {
        switch alertData {
        case .none:
            EmptyView()
        case .errorData(let value):
            ErrorAlert(pushButton: pushButton, content: value)
        case .shield(let value):
            ShieldAlertComponent(resetAlert: resetAlert, pushButton: pushButton, canaryDead: canaryDead, content: value)
        case .confirm:// (let value):
            let value = "TODO"
            ConfirmAlert(pushButton: pushButton, content: value)
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
