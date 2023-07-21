// swift-tools-version: 5.7

import PackageDescription

let package = Package(
    name: "PolkadotIdenticon",
    platforms: [
        .iOS(.v15)
    ],
    products: [
        .library(
            name: "PolkadotIdenticon",
            targets: ["PolkadotIdenticon"]
        )
    ],
    dependencies: [.package(url: "https://github.com/tesseract-one/Blake2.swift.git", from: "0.1.0")],
    targets: [
        .target(
            name: "PolkadotIdenticon",
            dependencies: [
                .product(name: "Blake2", package: "Blake2.swift")
            ]
        ),
        .testTarget(
            name: "PolkadotIdenticonTests",
            dependencies: ["PolkadotIdenticon"]
        )
    ]
)
