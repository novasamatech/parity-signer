//
//  TCDerivations.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 6.1.2022.
//

import SwiftUI

struct TCDerivations: View {
    let value: [SeedKeysPreview]
    var body: some View {
        VStack(spacing: Spacing.extraSmall) {
            ForEach(value, id: \.self) { singleKey($0) }
            ForEach(value, id: \.self) { singleKey($0) }
        }
    }

    @ViewBuilder
    func singleKey(_ preview: SeedKeysPreview) -> some View {
        VStack(alignment: .leading, spacing: 0) {
            // Root key
            VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                Text(preview.name)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(PrimaryFont.titleM.font)
                Text(preview.multisigner?.first ?? "address")
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.bodyM.font)
            }
            // Derived key header
            Localizable.ImportKeys.Label.Key.derivedKey.text
                .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                .font(PrimaryFont.bodyM.font)
                .padding(.vertical, Spacing.medium)
            // Derived keys list
            ForEach(preview.derivedKeys, id: \.self) {
                derivedKey($0)
                if $0 != preview.derivedKeys.last {
                    Divider()
                }
            }
        }
        .padding(Spacing.medium)
        .containerBackground()
    }

    @ViewBuilder
    func derivedKey(_ preview: DerivedKeyPreview) -> some View {
        HStack(alignment: .center, spacing: Spacing.small) {
            Identicon(identicon: PreviewData.exampleIdenticon, rowHeight: Heights.identiconInCell)
            VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                fullPath(preview)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                Text(preview.address)
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            }
            .font(PrimaryFont.bodyM.font)
            .padding(.vertical, Spacing.extraSmall)
        }
    }

    /// String interpolation for SFSymbols is a bit unstable if creating `String` inline by using conditional logic or
    /// `appending` from `StringProtocol`. Hence less DRY approach and dedicated function to wrap that
    private func fullPath(_ preview: DerivedKeyPreview) -> Text {
        true ?
            Text(
                "\(preview.displayablePath)\(Image(.lock))"
            ) :
            Text(preview.displayablePath)
    }
}

private extension DerivedKeyPreview {
    /// Returns either `path` or if password protected, available path with path delimeter and lock icon
    var displayablePath: String {
        true ?
            "\(derivationPath ?? "")\(Localizable.Address.Label.PasswordProtectedPath.pathDelimeter.string)" :
            derivationPath ?? ""
    }
}

struct TCDerivations_Previews: PreviewProvider {
    static var previews: some View {
        TCDerivations(value: [SeedKeysPreview(name: "Derivation 1", multisigner: nil, derivedKeys: [])])
    }
}
