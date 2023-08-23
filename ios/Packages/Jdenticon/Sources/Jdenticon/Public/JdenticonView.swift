//
//  JdenticonView.swift
//
//
//  Created by Krzysztof Rodak on 21/08/2023.
//

import SwiftUI

public struct JdenticonView: View {
    let generator: JdenticonGenerator
    let hash: Data
    let size: CGFloat

    public init(generator: JdenticonGenerator = JdenticonGenerator(), hash: Data, size: CGFloat) {
        self.generator = generator
        self.hash = hash
        self.size = size
    }

    public var body: some View {
        Image(uiImage: createIdenticonImage())
            .resizable()
            .frame(width: size, height: size)
            .aspectRatio(contentMode: .fit)
    }

    private func createIdenticonImage() -> UIImage {
        generator.render(size: size, hash: hash)
    }
}
