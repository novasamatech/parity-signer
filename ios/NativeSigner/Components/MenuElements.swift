//
//  MenuElements.swift
//  NativeSigner
//
//  Created by Sveta Goldstein on 16.12.2021.
//

import SwiftUI

struct MenuButtonsStack<Content: View>: View {
    let content: Content
    init(@ViewBuilder content: () -> Content) {
        self.content = content()
    }
    var body: some View {
        VStack (spacing: 12) {
            self.content
        }
        .padding(.top, 10)
    }
}
