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
    let navigationRequest: NavigationRequest

    var body: some View {
        switch alertData {
        case .none:
            EmptyView()
        case let .errorData(value):
            ErrorAlert(navigationRequest: navigationRequest, content: value)
        case let .shield(value):
            ShieldAlertComponent(
                resetAlert: resetAlert,
                navigationRequest: navigationRequest,
                canaryDead: canaryDead,
                content: value
            )
        case .confirm:
            let value = "TODO"
            ConfirmAlert(navigationRequest: navigationRequest, content: value)
        }
    }
}

// struct AlertSelector_Previews: PreviewProvider {
//    static var previews: some View {
//        AlertSelector()
//    }
// }
