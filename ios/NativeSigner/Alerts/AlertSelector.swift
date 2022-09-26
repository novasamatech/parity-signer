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
    let isConnectivityOn: Bool
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
                isConnectivityOn: isConnectivityOn,
                content: value
            )
        case .confirm:
            ConfirmAlert(
                navigationRequest: navigationRequest,
                content: "TODO"
            )
        }
    }
}

// struct AlertSelector_Previews: PreviewProvider {
//    static var previews: some View {
//        AlertSelector()
//    }
// }
