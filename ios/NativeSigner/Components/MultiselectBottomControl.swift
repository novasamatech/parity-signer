//
//  MultiselectBottomControl.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.10.2021.
//

import SwiftUI

/// Panel with actions for multiselect
struct MultiselectBottomControl: View {
    @State private var delete = false
    var selectedCount: String
    var navigationRequest: NavigationRequest
    var body: some View {
        ZStack {
            HStack {
                Button(
                    action: {
                        delete = true
                    },
                    label: {
                        SmallButton(text: "Delete")
                    }
                )
                .disabled(selectedCount == "0")
                .alert(isPresented: $delete, content: {
                    Alert(
                        title: Text("Delete key?"),
                        message: Text("You are about to delete selected keys"),
                        primaryButton: .cancel(),
                        secondaryButton: .destructive(
                            Text("Delete"),
                            action: {
                                navigationRequest(.init(action: .removeKey))
                            }
                        )
                    )
                })
                Spacer()
                Button(
                    action: {
                        navigationRequest(.init(action: .exportMultiSelect))
                    },
                    label: {
                        SmallButton(text: "Export")
                    }
                ).disabled(selectedCount == "0")
            }
            HStack {
                Text(selectedCount)
                Text("items selected")
            }
        }
        .padding(.vertical)
    }
}

// struct MultiselectBottomControl_Previews: PreviewProvider {
// static var previews: some View {
// MultiselectBottomControl()
// }
// }
