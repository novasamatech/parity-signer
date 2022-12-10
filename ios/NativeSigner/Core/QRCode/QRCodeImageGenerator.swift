//
//  QRCodeImageGenerator.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 19/10/2022.
//

import QRCode
import UIKit

final class QRCodeImageGenerator {
    private let doc: QRCode.Document

    init() {
        doc = QRCode.Document()
        doc.errorCorrection = .default
        doc.design.backgroundColor(UIColor.clear.cgColor)
        doc.design.foregroundColor(UIColor.black.cgColor)
        doc.design.shape.eye = QRCode.EyeShape.Squircle()
        doc.design.shape.onPixels = QRCode.PixelShape.Circle()
    }

    func generateQRCode(from bytes: [UInt8]) -> UIImage {
        doc.data = Data(bytes)
        return doc.uiImage(CGSize(width: 800, height: 800)) ?? UIImage()
    }
}
