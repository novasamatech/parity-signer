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
        VStack(spacing: 12) {
            self.content
        }
        .padding(.top, 10)
    }
}

struct MenuStack<Content: View>: View {
    let content: Content
    init(@ViewBuilder content: () -> Content) {
        self.content = content()
    }
    var body: some View {
        VStack {
            Spacer()
            VStack(alignment: .leading) {
                self.content
            }
            .padding(.top, 6.0)
            .padding(.horizontal, 12)
            .padding(.bottom, 24.0)
            .background(Color("Bg000"))
            .cornerRadius(radius: 8, corners: [.topLeft, .topRight])
            .shadow(color: Color(red: 0.0, green: 0.0, blue: 0.0, opacity: 0.2), radius: 32, x: 0, y: -16)
        }
    }
}
