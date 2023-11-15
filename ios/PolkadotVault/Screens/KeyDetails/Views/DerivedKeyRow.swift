//
//  DerivedKeyRow.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 30/08/2022.
//

import SwiftUI

struct DerivedKeyRow: View {
    let viewModel: DerivedKeyRowViewModel

    var body: some View {
        HStack(alignment: .center, spacing: Spacing.small) {
            NetworkIdenticon(
                identicon: viewModel.identicon,
                network: viewModel.network,
                background: .backgroundPrimary,
                size: Heights.identiconInCell
            )
            .padding(.top, Spacing.extraExtraSmall)
            .padding(.leading, Spacing.medium)
            VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                if !isRoot {
                    fullPath
                        .foregroundColor(.textAndIconsTertiary)
                        .font(PrimaryFont.captionM.font)
                }
                HStack(spacing: Spacing.extraExtraSmall) {
                    Text(viewModel.base58.truncateMiddle())
                        .foregroundColor(.textAndIconsPrimary)
                        .font(PrimaryFont.bodyL.font)
                        .lineLimit(1)
                }
            }
            Spacer()
            VStack(alignment: .center) {
                Image(.chevronRight)
                    .foregroundColor(.textAndIconsSecondary)
            }
            .padding(.trailing, Spacing.large)
            .frame(minHeight: .zero, maxHeight: .infinity)
        }
        .padding([.top, .bottom], Spacing.medium)
        .fixedSize(horizontal: false, vertical: true)
    }

    /// String interpolation for SFSymbols is a bit unstable if creating `String` inline by using conditional logic or
    /// `appending` from `StringProtocol`. Hence less DRY approach and dedicated function to wrap that
    private var fullPath: Text {
        viewModel.hasPassword ?
            Text(
                "\(viewModel.path)\(Localizable.Shared.Label.passwordedPathDelimeter.string)\(Image(.lock))"
            ) :
            Text(viewModel.path)
    }

    private var isRoot: Bool {
        !viewModel.hasPassword && viewModel.path.isEmpty
    }
}

#if DEBUG
    struct DerivedKeyRow_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                DerivedKeyRow(
                    viewModel: DerivedKeyRowViewModel(
                        identicon: .stubIdenticon,
                        network: "polkadot",
                        path: "//polkadot",
                        hasPassword: false,
                        base58: "15Gsc678654FDSG0HA04H0A",
                        rootKeyName: ""
                    )
                )
                DerivedKeyRow(
                    viewModel: DerivedKeyRowViewModel(
                        identicon: .stubIdenticon,
                        network: "kusama",
                        path: "",
                        hasPassword: false,
                        base58: "15Gsc678654FDSG0HA04H0A"
                    )
                )
                DerivedKeyRow(
                    viewModel: DerivedKeyRowViewModel(
                        identicon: .stubIdenticon,
                        network: "astar",
                        path: "//astar",
                        hasPassword: false,
                        base58: "15Gsc678654FDSG0HA04H0A"
                    )
                )
                DerivedKeyRow(
                    viewModel: DerivedKeyRowViewModel(
                        identicon: .stubIdenticon,
                        network: "kusama",
                        path: "//kusama",
                        hasPassword: true,
                        base58: "15Gsc678654FDSG0HA04H0A"
                    )
                )
                DerivedKeyRow(
                    viewModel: DerivedKeyRowViewModel(
                        identicon: .stubIdenticon,
                        network: "kusama",
                        path: "//kusama//verylongpathsolongitrequirestwolinesoftextormaybeevenmoremaybethree",
                        hasPassword: true,
                        base58: "15Gsc678654FDSG0HA04H0A"
                    )
                )
            }
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
        }
    }
#endif
