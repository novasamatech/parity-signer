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
                        SmallButton(text: Localizable.delete.key)
                    }
                )
                .disabled(selectedCount == "0")
                .alert(isPresented: $delete, content: {
                    Alert(
                        title: Localizable.deleteKey.text,
                        message: Localizable.youAreAboutToDeleteSelectedKeys.text,
                        primaryButton: .cancel(),
                        secondaryButton: .destructive(
                            Localizable.delete.text,
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
                        SmallButton(text: Localizable.export.key)
                    }
                ).disabled(selectedCount == "0")
            }
            HStack {
                Text(selectedCount)
                Localizable.itemsSelected.text
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
