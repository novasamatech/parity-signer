//
//  AlertSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.12.2021.
//

import CoreML
import SwiftUI

struct AlertSelector: View {
    let alertData: AlertData?
    let canaryDead: Bool
    let resetAlert: () -> Void
    let pushButton: (Action, String, String) -> Void

    var body: some View {
        switch alertData {
        case .none:
            EmptyView()
        case let .errorData(value):
            ErrorAlert(pushButton: pushButton, content: value)
        case let .shield(value):
            ShieldAlertComponent(resetAlert: resetAlert, pushButton: pushButton, canaryDead: canaryDead, content: value)
        case .confirm: // (let value):
            let value = "TODO"
            ConfirmAlert(pushButton: pushButton, content: value)
        }
    }
}

// struct AlertSelector_Previews: PreviewProvider {
//    static var previews: some View {
//        AlertSelector()
//    }
// }
