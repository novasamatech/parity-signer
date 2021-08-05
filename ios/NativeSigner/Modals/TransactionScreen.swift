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
                    CameraView()
                }
            case .preview :
                VStack {
                    Text("Preview")
                }
            case .show :
                VStack {
                    Text("Show QR")
                }
            default:
                VStack {
                    Text("Please wait")
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
