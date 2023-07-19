// swift-tools-version: 5.8

import PackageDescription

let package = Package(
    name: "Blockies",
    platforms: [
        .iOS(.v15)
    ],
    products: [
        .library(
            name: "Blockies",
            targets: ["Blockies"]
        )
    ],
    dependencies: [],
    targets: [
        .target(
            name: "Blockies",
            dependencies: []
        ),
        .testTarget(
            name: "BlockiesTests",
            dependencies: ["Blockies"]
        )
    ]
)
