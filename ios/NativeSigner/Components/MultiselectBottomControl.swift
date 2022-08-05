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
    @State var delete = false
    var selectedCount: String
    var pushButton: (Action, String, String) -> Void
    var body: some View {
        ZStack {
            HStack {
                Button(
                    action: {
                        delete = true
                    },
                    label: {
                        SmallButton(text: "Delete")
                    })
                    .disabled(selectedCount == "0")
                    .alert(isPresented: $delete, content: {
                        Alert(
                            title: Text("Delete key?"),
                            message: Text("You are about to delete selected keys"),
                            primaryButton: .cancel(),
                            secondaryButton: .destructive(
                                Text("Delete"),
                                action: {
                                    pushButton(.removeKey, "", "")
                                }
                            )
                        )
                    })
                Spacer()
                Button(
                    action: {
                        pushButton(.exportMultiSelect, "", "")
                    },
                    label: {
                        SmallButton(text: "Export")
                    }).disabled(selectedCount == "0")
            }
            HStack {
                Text(selectedCount)
                Text("items selected")
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
