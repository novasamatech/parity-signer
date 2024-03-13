// swift-tools-version: 5.9

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
    dependencies: [
        .package(url: "https://github.com/tesseract-one/Blake2.swift.git", from: "0.1.0"),
        .package(url: "https://github.com/attaswift/BigInt.git", from: "5.3.0")
    ],
    targets: [
        .target(
            name: "PolkadotIdenticon",
            dependencies: [
                .product(name: "Blake2", package: "Blake2.swift"),
                .product(name: "BigInt", package: "BigInt")
            ]
        ),
        .testTarget(
            name: "PolkadotIdenticonTests",
            dependencies: ["PolkadotIdenticon"]
        )
    ]
)
