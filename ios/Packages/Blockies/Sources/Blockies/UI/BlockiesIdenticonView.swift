//
//  BlockiesIdenticonView.swift
//
//
//  Created by Krzysztof Rodak on 19/07/2023.
//

import SwiftUI

public struct BlockiesIdenticonView: View {
    let configuration: BlockiesConfiguration
    let width: CGFloat
    let height: CGFloat

    public init(configuration: BlockiesConfiguration, width: CGFloat, height: CGFloat) {
        self.configuration = configuration
        self.width = width
        self.height = height
    }

    public var body: some View {
        if let identiconImage = createIdenticonImage() {
            Image(uiImage: identiconImage)
                .resizable()
                .frame(width: width, height: height)
                .aspectRatio(contentMode: .fit)
        } else {
            EmptyView()
        }
    }

    private func createIdenticonImage() -> UIImage? {
        let customScale = max(Int(width / CGFloat(configuration.size)), 1)
        let generator = BlockiesIdenticonGenerator(configuration: configuration)
        return generator.createImage(customScale: customScale)
    }
}
