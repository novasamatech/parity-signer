//
//  BlockiesConfiguration.swift
//
//
//  Created by Krzysztof Rodak on 17/07/2023.
//

import Foundation

/// A structure that holds the configuration settings for a Blockies identicon.
///
/// This struct encapsulates the size and scale for the identicon. The `size` determines
/// the number of blocks along the x and y axis of the image, while `scale` specifies
/// the pixel size of each block.
public struct BlockiesConfiguration {
    /// The size of the identicon.
    ///
    /// This value represents the number of blocks along the x and y axis of the identicon.
    /// For example, a size of `8` will create an identicon that is `8` blocks wide and `8` blocks tall.
    public let size: Int

    /// The scale of the identicon.
    ///
    /// This value represents the pixel size of each block in the identicon.
    /// For example, a scale of `3` will create each block with a size of `3x3` pixels.
    public let scale: Int

    /// Initializes a new instance of a Blockies configuration.
    ///
    /// This initializer takes a size and a scale which are used to create the identicon.
    ///
    /// - Parameters:
    ///   - size: The number of blocks along the x and y axis of the identicon. Default value is `8`.
    ///   - scale: The pixel size of each block in the identicon. Default value is `3`.
    public init(
        size: Int = 8,
        scale: Int = 3
    ) {
        self.size = size
        self.scale = scale
    }

    /// A static configuration that is suitable for Ethereum addresses.
    ///
    /// This configuration creates an identicon that is `8` blocks wide and `8` blocks tall,
    /// with each block having a size of `3x3` pixels.
    public static let ethAddress = BlockiesConfiguration(size: 8, scale: 3)
}
