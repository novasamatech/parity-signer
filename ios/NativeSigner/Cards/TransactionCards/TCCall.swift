//
//  TCCall.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCCall: View {
    let value: MscCall
    @State private var showDoc = false
    var body: some View {
        Button(
            action: {
                self.showDoc.toggle()
            },
            label: {
                VStack {
                    HStack {
                        TCNamedValueCard(name: Localizable.TCName.method.string, value: value.methodName)
                        if !value.docs.isEmpty {
                            Localizable.questionMark.text
                                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                        }
                    }
                    if showDoc {
                        Text.markdownWithFallback(value.docs)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            .multilineTextAlignment(.leading)
                    }
                }
                .font(Fontstyle.bodyL.base)
            }
        )
        .disabled(value.docs.isEmpty)
    }
}

struct TCCall_Previews: PreviewProvider {
    static var previews: some View {
        TCCall(value: MscCall(methodName: "method name", docs: "docs"))
    }
}
