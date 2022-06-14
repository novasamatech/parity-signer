//
//  TransactionScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 5.8.2021.
//

import SwiftUI

struct TransactionScreen: View {
    let pushButton: (Action, String, String) -> Void
    var body: some View {
        CameraView(pushButton: pushButton)
    }
}

/*
struct TransactionScreen_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            TransactionScreen()
        }
    }
}
*/
