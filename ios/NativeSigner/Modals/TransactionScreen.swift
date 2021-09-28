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
        ZStack {
            switch (data.transactionState) {
            case .none :
                VStack {
                    CameraView()
                }
            case .preview :
                VStack {
                    TransactionPreview()
                }
            case .signed :
                TransactionReady()
            default:
                VStack {
                    Text("Please wait")
                        .foregroundColor(Color("textMainColor"))
                }
            }
        }.background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
    }
}

struct TransactionScreen_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            TransactionScreen()
        }
    }
}
