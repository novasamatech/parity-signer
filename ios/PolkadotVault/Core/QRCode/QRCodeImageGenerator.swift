//
//  QRCodeImageGenerator.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 19/10/2022.
//

import QRCode
import SwiftUI
import UIKit

final class QRCodeImageGenerator {
    enum Style {
        case `private`
        case `public`
    }

    private let doc: QRCode.Document

    init() {
        doc = QRCode.Document()
        doc.errorCorrection = .default
        doc.design.backgroundColor(UIColor.clear.cgColor)
        doc.design.shape.eye = QRCode.EyeShape.Squircle()
        doc.design.style.eye = QRCode.FillStyle.Solid(UIColor.black.cgColor)
        doc.design.shape.onPixels = QRCode.PixelShape.Circle()
    }

    func generateQRCode(from bytes: [UInt8], style: QRCodeImageGenerator.Style = .public) -> UIImage {
        style.apply(doc)
        doc.data = Data(bytes)
        return doc.uiImage(CGSize(width: 800, height: 800)) ?? UIImage()
    }
}

extension QRCodeImageGenerator.Style {
    func apply(_ doc: QRCode.Document) {
        switch self {
        case .public:
            doc.design.foregroundColor(UIColor.black.cgColor)
        case .private:
            doc.design.foregroundColor(UIColor(Color(.accentPink500)).cgColor)
        }
    }
}
