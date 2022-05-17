//
//  AddressCardControls.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 15.10.2021.
//

import SwiftUI

struct AddressCardControls: View {
    @EnvironmentObject var data: SignerDataModel
    var seed_name: String
    var rowHeight: CGFloat = 39
    @State private var delete = false
    @State private var count: CGFloat = 1
    var body: some View {
        HStack {
            Spacer()
            Button(action: {
                let seed_phrase = data.getSeed(seedName: seed_name)
                if seed_phrase != "" {
                    data.pushButton(action: .increment, details: "1", seedPhrase: seed_phrase)
                }
            }) {
                ZStack {
                    RoundedRectangle(cornerRadius: 6).foregroundColor(Color("Crypto100"))
                    Text("N+"+String(Int(count))).font(FCrypto(style: .body2)).foregroundColor(Color("Crypto400"))
                }
                .frame(width: rowHeight, height: rowHeight)
                .gesture(DragGesture()
                            .onChanged{drag in
                    count = exp(abs(drag.translation.height)/50)
                }
                            .onEnded{_ in
                    let seed_phrase = data.getSeed(seedName: seed_name)
                    if seed_phrase != "" {
                        data.pushButton(action: .increment, details: String(Int(count)), seedPhrase: seed_phrase)
                    }
                })
                .onAppear {
                    count = 1
                }
            }
            Button(action: {
                delete = true
            }) {
                ZStack {
                    RoundedRectangle(cornerRadius: 6).foregroundColor(Color("SignalDanger"))
                    Image(systemName: "trash.slash").foregroundColor(Color("BgDanger"))
                }
                .frame(width: rowHeight, height: rowHeight)
                .alert(isPresented: $delete, content: {
                    Alert(
                        title: Text("Delete key?"),
                        message: Text("You are about to delete key"),
                        primaryButton: .cancel(),
                        secondaryButton: .destructive(
                            Text("Delete"),
                            action: { data.pushButton(action: .removeKey)
                            }
                        )
                    )
                })
            }
        }
    }
}


/*
 struct AddressCardControls_Previews: PreviewProvider {
 static var previews: some View {
 AddressCardControls()
 }
 }
 */
