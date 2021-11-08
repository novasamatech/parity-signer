//
//  MultiselectBottomControl.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.10.2021.
//

import SwiftUI

/**
 * Panel with actions for multiselect
 */
struct MultiselectBottomControl: View {
    @EnvironmentObject var data: SignerDataModel
    @State var delete = false
    var body: some View {
        ZStack {
            HStack {
                Button(action: {
                    delete = true
                }) {
                    Text("Delete")
                }
                .alert(isPresented: $delete, content: {
                    Alert(
                        title: Text("Delete key?"),
                        message: Text("You are about to delete selected keys"),
                        primaryButton: .cancel(),
                        secondaryButton: .destructive(
                            Text("Delete"),
                            action: {
                                let cruncher = data.multiSelected
                                data.multiSelected = []
                                for i in cruncher {
                                    data.selectedAddress = i
                                    data.deleteSelectedAddress()
                                }
                            }
                        )
                    )
                })
                Spacer()
                Button(action: {
                    data.selectedAddress = data.multiSelected.first
                    data.keyManagerModal = .showKey
                }) {
                    Text("Export")
                }
            }
            HStack {
                Text(String(data.multiSelected.count))
                Text("addresses selected")
            }
        }
        .padding(.vertical)
    }
}

/*
 struct MultiselectBottomControl_Previews: PreviewProvider {
 static var previews: some View {
 MultiselectBottomControl()
 }
 }
 */
