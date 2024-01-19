//
//  BlockiesImageRenderer.swift
//
//
//  Created by Krzysztof Rodak on 17/07/2023.
//

#if os(iOS) || os(tvOS) || os(watchOS)
    import UIKit
#elseif os(OSX)
    import CoreGraphics
#endif

#if os(iOS) || os(tvOS) || os(watchOS)
    public typealias PlatformImage = UIImage
#elseif os(OSX)
    public typealias PlatformImage = NSImage
#endif

public class BlockiesImageRenderer {
    private let randomNumberGenerator: PseudoRandomNumberGenerator

    /// Initializes a new `BlockiesImageRenderer` instance.
    /// - Parameters:
    ///   - randomNumberGenerator: A pseudo random number generator used for generating fallback color values.
    public init(
        randomNumberGenerator: PseudoRandomNumberGenerator
    ) {
        self.randomNumberGenerator = randomNumberGenerator
    }

    /// Renders an image based on the given data and colors.
    ///
    /// - Parameters:
    ///   - data: The data that describes the blocks in the blockies.
    ///   - configuration: The Blockies configuration containing size and scale for the image.
    ///   - colors: The `ColorsConfiguration` object containing the primary, background, and spot color for the
    /// blockies.
    ///   - scalingFactor: The scaling factor for the size of the blocks.
    ///
    /// - returns: The rendered image, or `nil` if the image could not be created.
    public func renderImage(
        from data: [Double],
        configuration: BlockiesConfiguration,
        colors: ColorsConfiguration,
        scalingFactor: Int
    ) -> PlatformImage? {
        let finalSize = configuration.size * configuration.scale * scalingFactor
        let primaryColor = colors.color
        let backgroundColor = colors.bgcolor
        let spotColor = colors.spotcolor

        #if os(iOS) || os(tvOS) || os(watchOS)
            let renderer = UIGraphicsImageRenderer(size: CGSize(width: finalSize, height: finalSize))
            let image = renderer.image { rendererContext in
                var context = rendererContext.cgContext
                fill(
                    context: &context,
                    from: data,
                    primaryColor: primaryColor,
                    backgroundColor: backgroundColor,
                    spotColor: spotColor,
                    size: configuration.size,
                    scale: configuration.scale,
                    scalingFactor: scalingFactor
                )
            }
            return image

        #elseif os(OSX)
            let colorSpace = CGColorSpaceCreateDeviceRGB()
            let bitmapInfo = CGBitmapInfo(rawValue: CGImageAlphaInfo.premultipliedLast.rawValue)
            var context = CGContext(
                data: nil,
                width: finalSize,
                height: finalSize,
                bitsPerComponent: 8,
                bytesPerRow: 0,
                space: colorSpace,
                bitmapInfo: bitmapInfo.rawValue
            )
            fill(
                context: &context,
                from: data,
                primaryColor: primaryColor,
                backgroundColor: backgroundColor,
                spotColor: spotColor,
                size: configuration.size,
                scale: configuration.scale,
                scalingFactor: scalingFactor
            )
            guard let outputCGImage = context.makeImage() else {
                return nil
            }
            return NSImage(cgImage: outputCGImage, size: CGSize(width: finalSize, height: finalSize))
        #endif
    }

    private func fill(
        context: inout CGContext,
        from data: [Double],
        primaryColor: PlatformColor,
        backgroundColor: PlatformColor,
        spotColor: PlatformColor,
        size: Int,
        scale: Int,
        scalingFactor: Int
    ) {
        let dataLength = Int(sqrt(Double(data.count)))

        context.setFillColor(backgroundColor.cgColor)
        context.fill(CGRect(
            x: 0,
            y: 0,
            width: size * scale,
            height: size * scale
        ))

        for index in 0 ..< data.count {
            let row = Int(floor(Double(index) / Double(dataLength)))
            let col = index % dataLength

            let number = data[index]

            let blockColor: PlatformColor =
                switch number {
                case 0:
                    backgroundColor
                case 1:
                    primaryColor
                case 2:
                    spotColor
                default:
                    PlatformColor.black
                }
            context.setFillColor(blockColor.cgColor)
            context.fill(CGRect(
                x: CGFloat(col * scale * scalingFactor),
                y: CGFloat(row * scale * scalingFactor),
                width: CGFloat(scale * scalingFactor),
                height: CGFloat(scale * scalingFactor)
            ))
        }
    }
}
