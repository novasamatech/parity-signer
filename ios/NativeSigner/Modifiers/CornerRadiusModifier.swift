//
//  CornerRadiusModifier.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 27/12/2022.
//

import SwiftUI

struct CornerRadiusStyle: ViewModifier {
    fileprivate let shape: CornerRadiusShape

    func body(content: Content) -> some View {
        content
            .clipShape(shape)
    }
}

private struct CornerRadiusShape: Shape {
    let radius: CGFloat
    let corners: UIRectCorner

    func path(in rect: CGRect) -> Path {
        let path = UIBezierPath(
            roundedRect: rect,
            byRoundingCorners: corners,
            cornerRadii: CGSize(width: radius, height: radius)
        )
        return Path(path.cgPath)
    }
}

extension View {
    func cornerRadius(radius: CGFloat = CornerRadius.medium, corners: UIRectCorner) -> some View {
        modifier(CornerRadiusStyle(shape: CornerRadiusShape(radius: radius, corners: corners)))
    }
}
