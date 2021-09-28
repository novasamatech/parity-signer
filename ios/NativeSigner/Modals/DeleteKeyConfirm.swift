//
//  DeleteKeyConfirm.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.9.2021.
//

import SwiftUI

/**
 * This is a sketch in case we need more complex key removal confirmation dialog
 */
struct DeleteKeyConfirm: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        ZStack{
            RoundedRectangle(cornerRadius: 50).foregroundColor(/*@START_MENU_TOKEN@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
            VStack {
                Text("Delete key?").font(.title)
                Text("You are about to delete this key").font(.headline)
                Button(action: {data.totalRefresh()}) {
                    Text("Done").font(.largeTitle)
                }
            }
        }
    }
}

struct DeleteKeyConfirm_Previews: PreviewProvider {
    static var previews: some View {
        DeleteKeyConfirm()
    }
}
