//
//  AddressCardControls.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 15.10.2021.
//

import SwiftUI

struct AddressCardControls: View {
    let seedName: String
    let increment: (String) -> Void
    let pushButton: (Action, String, String) -> Void
    var rowHeight: CGFloat = 39
    @State private var delete = false
    @State private var count: CGFloat = 1
    var body: some View {
        HStack {
            Spacer()
            Button(
                action: {
                    increment("1")
                },
                label: {
                    ZStack {
                        RoundedRectangle(cornerRadius: 6).foregroundColor(Color("Crypto100"))
                        Text("N+"+String(Int(count))).font(FCrypto(style: .body2)).foregroundColor(Color("Crypto400"))
                    }
                    .frame(width: rowHeight, height: rowHeight)
                    .gesture(
                        DragGesture()
                            .onChanged {drag in
                                count = exp(abs(drag.translation.height)/50)
                            }
                            .onEnded {_ in
                                increment(String(Int(count)))
                            }
                    )
                    .onAppear {
                        count = 1
                    }
                }
            )
            Button(
                action: {
                    delete = true
                },
                label: {
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
                                action: { pushButton(.removeKey, "", "") }
                            )
                        )
                    })
                })
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
