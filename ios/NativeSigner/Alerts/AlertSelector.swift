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
    let navigationRequest: NavigationRequest

    var body: some View {
        switch alertData {
        case .none:
            EmptyView()
        case let .errorData(value):
            ErrorAlert(navigationRequest: navigationRequest, content: value)
        case .shield:
            // Handled elsewhere
            EmptyView()
        case .confirm:
            // This was never used
            EmptyView()
        }
    }
}

// struct AlertSelector_Previews: PreviewProvider {
//    static var previews: some View {
//        AlertSelector()
//    }
// }
