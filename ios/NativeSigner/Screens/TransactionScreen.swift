//
//  TransactionScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 5.8.2021.
//

import SwiftUI

struct TransactionScreen: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        CameraView()
        Text("Camera")
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
