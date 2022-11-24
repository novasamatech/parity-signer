//
//  QRCodeImageGenerator.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 19/10/2022.
//

import CoreImage.CIFilterBuiltins
import UIKit

final class QRCodeImageGenerator {
    private let context = CIContext()
    private let filter = CIFilter.qrCodeGenerator()

    func generateQRCode(from bytes: [UInt8]) -> UIImage {
        filter.message = Data(bytes)
        guard let outputImage = filter.outputImage,
              let cgimg = context.createCGImage(outputImage, from: outputImage.extent) else {
            return UIImage()
        }
        return UIImage(cgImage: cgimg)
    }
}
