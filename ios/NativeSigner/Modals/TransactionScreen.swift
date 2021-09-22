//
//  TransactionScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 5.8.2021.
//

import SwiftUI

struct TransactionScreen: View {
    @StateObject var transaction = Transaction()
    var body: some View {
        ZStack {
            switch (transaction.state) {
            case .scanning :
                VStack {
                    CameraView(transaction: transaction)
                }
            case .preview :
                VStack {
                    TransactionPreview(transaction: transaction)
                }
            case .show :
                TransactionReady(transaction: transaction)
            default:
                VStack {
                    Text("Please wait")
                        .foregroundColor(Color("textMainColor"))
                }
            }
        }
        .navigationTitle("Transaction").navigationBarTitleDisplayMode(.inline).toolbar {
            ToolbarItem(placement: .navigationBarTrailing) {
                NavbarShield()
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
