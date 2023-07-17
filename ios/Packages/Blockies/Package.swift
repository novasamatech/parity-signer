// swift-tools-version: 5.8
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Blockies",
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
