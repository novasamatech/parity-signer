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
    let navigationRequest: NavigationRequest
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
                        RoundedRectangle(cornerRadius: 6).foregroundColor(Asset.accentPink300.swiftUIColor)
                        Text("N+" + String(Int(count))).font(PrimaryFont.captionM.font)
                            .foregroundColor(Asset.accentPink300.swiftUIColor)
                    }
                    .frame(width: rowHeight, height: rowHeight)
                    .gesture(
                        DragGesture()
                            .onChanged { drag in
                                count = exp(abs(drag.translation.height) / 50)
                            }
                            .onEnded { _ in
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
                        RoundedRectangle(cornerRadius: 6).foregroundColor(Asset.accentRed400.swiftUIColor)
                        Image(.trash, variant: .slash).foregroundColor(Asset.accentRed300.swiftUIColor.opacity(0.3))
                    }
                    .frame(width: rowHeight, height: rowHeight)
                    .alert(isPresented: $delete, content: {
                        Alert(
                            title: Localizable.deleteKey.text,
                            message: Localizable.youAreAboutToDeleteKey.text,
                            primaryButton: .cancel(),
                            secondaryButton: .destructive(
                                Localizable.delete.text,
                                action: { navigationRequest(.init(action: .removeKey)) }
                            )
                        )
                    })
                }
            )
        }
    }
}

// struct AddressCardControls_Previews: PreviewProvider {
// static var previews: some View {
// AddressCardControls()
// }
// }
