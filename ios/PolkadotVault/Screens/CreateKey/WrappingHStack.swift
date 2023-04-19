//
//  WrappingHStack.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 27/02/2023.
//

import SwiftUI

// swiftlint:disable all
struct WrappingHStack<Model, V>: View where Model: Hashable & Identifiable, V: View {
    typealias ViewGenerator = (Model) -> V

    var models: [Model]
    @ViewBuilder
    var viewGenerator: ViewGenerator
    var horizontalSpacing: CGFloat = 0
    var verticalSpacing: CGFloat = 0

    @State private var totalHeight
        = CGFloat.zero // << variant for ScrollView/List
//        = CGFloat.infinity // << variant for VStack

    var body: some View {
        VStack {
            GeometryReader { geometry in
                generateContent(in: geometry)
            }
        }
        .frame(height: totalHeight) // << variant for ScrollView/List
//        .frame(maxHeight: totalHeight) // << variant for VStack
    }

    private func generateContent(in geometry: GeometryProxy) -> some View {
        var width = CGFloat.zero
        var height = CGFloat.zero

        return ZStack(alignment: .topLeading) {
            ForEach(models, id: \.id) { models in
                viewGenerator(models)
                    .padding(.horizontal, horizontalSpacing)
                    .padding(.vertical, verticalSpacing)
                    .alignmentGuide(.leading, computeValue: { dimension in
                        if abs(width - dimension.width) > geometry.size.width {
                            width = 0
                            height -= dimension.height
                        }
                        let result = width
                        if models == self.models.last! {
                            width = 0 // last item
                        } else {
                            width -= dimension.width
                        }
                        return result
                    })
                    .alignmentGuide(.top, computeValue: { _ in
                        let result = height
                        if models == self.models.last! {
                            height = 0 // last item
                        }
                        return result
                    })
            }
        }
        .background(viewHeightReader($totalHeight))
    }

    private func viewHeightReader(_ binding: Binding<CGFloat>) -> some View {
        GeometryReader { geometry -> Color in
            let rect = geometry.frame(in: .local)
            DispatchQueue.main.async {
                binding.wrappedValue = rect.size.height
            }
            return .clear
        }
    }
}
