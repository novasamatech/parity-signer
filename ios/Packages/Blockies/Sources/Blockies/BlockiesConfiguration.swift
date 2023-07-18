//
//  BlockiesConfiguration.swift
//
//
//  Created by Krzysztof Rodak on 17/07/2023.
//

import Foundation

public struct BlockiesConfiguration {
    public let seed: String
    public let color: PlatformColor?
    public let bgcolor: PlatformColor?
    public let spotcolor: PlatformColor?
    public let size: Int
    public let scale: Int

    public init(
        seed: String?,
        color: PlatformColor?,
        bgcolor: PlatformColor?,
        spotcolor: PlatformColor?,
        size: Int = 8,
        scale: Int = 4
    ) {
        self.seed = seed ?? String(Int64(floor(Double.random * pow(10, 16))))
        self.color = color
        self.bgcolor = bgcolor
        self.spotcolor = spotcolor
        self.size = size
        self.scale = scale
    }
}

extension Double {
    static var random: Double {
        Double(arc4random()) / Double(UInt32.max)
    }
}
