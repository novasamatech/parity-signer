//
//  TransactionScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 5.8.2021.
//

import SwiftUI

struct TransactionScreen: View {
    let navigationRequest: NavigationRequest
    var body: some View {
        CameraView(navigationRequest: navigationRequest)
    }
}

// struct TransactionScreen_Previews: PreviewProvider {
//    static var previews: some View {
//        NavigationView {
//            TransactionScreen()
//        }
//    }
// }
