//
//  TCDerivations.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 6.1.2022.
//

import SwiftUI

struct DerivedKeysSetRenderable: Equatable, Identifiable {
    struct DerivedKeyRenderable: Equatable, Identifiable {
        let id = UUID()
        let identicon: SignerImage
        let derivationPath: String
        let isPassworded: Bool
        let address: String
        let networkTitle: String?
    }

    let id = UUID()
    let seedName: String
    let address: String
    let keys: [DerivedKeyRenderable]
}

struct TCDerivations: View {
    @Binding var value: [SeedKeysPreview]
    @StateObject var viewModel: ViewModel

    var body: some View {
        errorStates()
            .padding(.horizontal, Spacing.medium)
        VStack(spacing: Spacing.extraSmall) {
            ForEach(viewModel.importableKeySets, id: \.id) { singleKey($0) }
        }
        .padding(.horizontal, Spacing.extraSmall)
        .onAppear {
            viewModel.updateData(value)
        }
        .onChange(of: value, perform: { newValue in
            viewModel.updateData(newValue)
        })
    }

    @ViewBuilder
    func singleKey(_ keySet: DerivedKeysSetRenderable) -> some View {
        VStack(alignment: .leading, spacing: 0) {
            // Root key
            VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                Text(keySet.seedName)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(PrimaryFont.titleS.font)
                Text(keySet.address)
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.bodyM.font)
            }
            .padding(.horizontal, Spacing.medium)
            .padding(.top, Spacing.medium)
            // Derived key header
            Text(Localizable.ImportKeys.Label.Title.derived(keySet.keys.count))
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                .font(PrimaryFont.bodyM.font)
                .padding(.top, Spacing.medium)
                .padding(.horizontal, Spacing.medium)
            // Derived keys list
            VStack(alignment: .leading, spacing: 0) {
                ForEach(keySet.keys, id: \.id) {
                    derivedKey($0)
                    if $0 != keySet.keys.last {
                        Divider()
                            .padding(.horizontal, Spacing.medium)
                    }
                }
                HStack {
                    Spacer()
                }
            }
            .containerBackground(CornerRadius.small)
            .padding(Spacing.extraSmall)
        }
        .containerBackground()
    }

    @ViewBuilder
    func derivedKey(_ preview: DerivedKeysSetRenderable.DerivedKeyRenderable) -> some View {
        VStack(alignment: .leading, spacing: Spacing.extraSmall) {
            HStack(alignment: .center, spacing: Spacing.extraSmall) {
                NetworkIdenticon(
                    identicon: preview.identicon,
                    network: preview.networkTitle,
                    background: Asset.fill6Solid.swiftUIColor,
                    size: Heights.identiconSmall
                )
                pathView(preview)
                    .font(PrimaryFont.bodyM.font)
                    .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                    .multilineTextAlignment(.leading)
                    .fixedSize(horizontal: false, vertical: true)
            }
            Text(preview.address)
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .multilineTextAlignment(.leading)
            if let networkTitle = preview.networkTitle {
                NetworkCapsuleView(network: networkTitle)
            }
        }
        .padding(Spacing.medium)
    }

    @ViewBuilder
    func errorStates() -> some View {
        VStack(spacing: Spacing.extraSmall) {
            if viewModel.isKeySetMissing {
                ActionableInfoBoxView(
                    renderable: .init(
                        text: Localizable.ImportKeys.Error.Label.keySetMissing.string
                    ),
                    action: nil
                )
            }
            if viewModel.isNetworkMissing {
                ActionableInfoBoxView(
                    renderable: .init(
                        text: Localizable.ImportKeys.Error.Label.networkMissing.string
                    ),
                    action: nil
                )
            }
            if viewModel.areKeysAlreadyImported {
                ActionableInfoBoxView(
                    renderable: .init(
                        text: Localizable.ImportKeys.Error.Label.alreadyImported.string
                    ),
                    action: nil
                )
            }
            if viewModel.isPathInBadFormat {
                ActionableInfoBoxView(
                    renderable: .init(
                        text: Localizable.ImportKeys.Error.Label.badFormat.string
                    ),
                    action: nil
                )
            }
        }
        .padding(.bottom, Spacing.medium)
    }

    @ViewBuilder
    private func pathView(_ renderable: DerivedKeysSetRenderable.DerivedKeyRenderable) -> some View {
        if renderable.derivationPath.isEmpty {
            Localizable.ImportKeys.Label.emptyPath.text
                .foregroundColor(Asset.textAndIconsDisabled.swiftUIColor)
        } else if renderable.isPassworded {
            Text(
                "\(renderable.displayablePath)\(Image(.lock))"
            )
        } else {
            Text(renderable.displayablePath)
        }
    }
}

private extension DerivedKeysSetRenderable.DerivedKeyRenderable {
    /// Returns either `path` or if password protected, available path with path delimeter and lock icon
    var displayablePath: String {
        isPassworded ?
            "\(derivationPath)\(Localizable.Shared.Label.passwordedPathDelimeter.string)" :
            derivationPath
    }
}

extension TCDerivations {
    final class ViewModel: ObservableObject {
        @Published var importableKeySets: [DerivedKeysSetRenderable] = []
        @Published var isKeySetMissing: Bool = false
        @Published var isNetworkMissing: Bool = false
        @Published var isPathInBadFormat: Bool = false
        @Published var areKeysAlreadyImported: Bool = false

        init() {}

        func updateData(_ value: [SeedKeysPreview]) {
            importableKeySets = value
                .map { seedKeys in
                    DerivedKeysSetRenderable(
                        seedName: seedKeys.name,
                        address: seedKeys.multisigner.first ?? "",
                        keys: seedKeys.derivedKeys
                            .filter { $0.status == .importable }
                            .filter { $0.hasPwd != nil }
                            .map { DerivedKeysSetRenderable
                                .DerivedKeyRenderable(
                                    identicon: $0.identicon,
                                    derivationPath: $0.derivationPath ?? "",
                                    isPassworded: $0.hasPwd == true,
                                    address: $0.address,
                                    networkTitle: $0.networkTitle
                                )
                            }
                    )
                }
            updateErrorStates(value)
        }

        private func updateErrorStates(_ value: [SeedKeysPreview]) {
            areKeysAlreadyImported = value
                .flatMap(\.derivedKeys)
                .contains { $0.status == .alreadyExists }
            isNetworkMissing = value
                .flatMap(\.derivedKeys)
                .contains {
                    if case let .invalid(error) = $0.status {
                        return error.contains(.networkMissing)
                    } else {
                        return false
                    }
                }
            isKeySetMissing = value
                .flatMap(\.derivedKeys)
                .contains {
                    if case let .invalid(error) = $0.status {
                        return error.contains(.keySetMissing)
                    } else {
                        return false
                    }
                }
            isPathInBadFormat = value
                .flatMap(\.derivedKeys)
                .contains {
                    if case let .invalid(error) = $0.status {
                        return error.contains(.badFormat)
                    } else {
                        return false
                    }
                }
        }
    }
}

#if DEBUG
    struct TCDerivations_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                TCDerivations(
                    value: .constant([
                        PreviewData.exampleSeedKeysPreview
                    ]),
                    viewModel: .init()
                )
            }
        }
    }

    extension PreviewData {
        static let exampleSeedKeysPreview = SeedKeysPreview(
            name: "Derivation 1",
            multisigner: ["long address", "encryption"],
            derivedKeys: [
                .init(
                    address: "address",
                    derivationPath: "//kusama",
                    encryption: .ed25519,
                    genesisHash: .init([3, 4, 5]),
                    identicon: .svg(image: PreviewData.exampleIdenticon),
                    hasPwd: nil,
                    networkTitle: "Kusama",
                    status: .alreadyExists
                ),
                .init(
                    address: "GD5434gFGFD543Dgdf",
                    derivationPath: "//westendMain",
                    encryption: .ed25519,
                    genesisHash: .init([3, 4, 5]),
                    identicon: .svg(image: PreviewData.exampleIdenticon),
                    hasPwd: true,
                    networkTitle: "Westend",
                    status: .invalid(errors: [.badFormat])
                ),
                .init(
                    address: "address",
                    derivationPath: "//polka",
                    encryption: .ed25519,
                    genesisHash: .init([3, 4, 5]),
                    identicon: .svg(image: PreviewData.exampleIdenticon),
                    hasPwd: false,
                    networkTitle: "Polkadot",
                    status: .importable
                ),
                .init(
                    address: "address",
                    derivationPath: "//polkadot//parachains",
                    encryption: .ed25519,
                    genesisHash: .init([3, 4, 5]),
                    identicon: .svg(image: PreviewData.exampleIdenticon),
                    hasPwd: true,
                    networkTitle: "Polkadot",
                    status: .importable
                ),
                .init(
                    address: "address",
                    derivationPath: "",
                    encryption: .ed25519,
                    genesisHash: .init([3, 4, 5]),
                    identicon: .svg(image: PreviewData.exampleIdenticon),
                    hasPwd: false,
                    networkTitle: nil,
                    status: .importable
                ),
                .init(
                    address: "address",
                    derivationPath: "//kusama//verylongpathsolongitrequirestwolinesoftextormaybeevenmoremaybethree",
                    encryption: .ed25519,
                    genesisHash: .init([3, 4, 5]),
                    identicon: .svg(image: PreviewData.exampleIdenticon),
                    hasPwd: true,
                    networkTitle: nil,
                    status: .importable
                )
            ]
        )
    }
#endif
