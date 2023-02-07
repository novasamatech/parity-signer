//
//  DerivedKeyOverviewRow.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 19/09/2022.
//

import SwiftUI

struct DerivedKeyOverviewViewModel: Equatable, Identifiable {
    let id = UUID()
    let identicon: [UInt8]
    let path: String
    let hasPassword: Bool
    let network: String

    init(
        identicon: [UInt8],
        path: String,
        hasPassword: Bool,
        network: String
    ) {
        self.identicon = identicon
        self.path = path
        self.hasPassword = hasPassword
        self.network = network
    }
}

extension DerivedKeyOverviewViewModel {
    init(_ key: MKeyAndNetworkCard) {
        path = key.key.address.path
        identicon = key.key.address.identicon.svgPayload
        hasPassword = key.key.address.hasPwd
        network = key.network.networkTitle
    }
}

struct DerivedKeyOverviewRow: View {
    private let viewModel: DerivedKeyOverviewViewModel

    init(_ viewModel: DerivedKeyOverviewViewModel) {
        self.viewModel = viewModel
    }

    var body: some View {
        HStack(alignment: .center, spacing: Spacing.small) {
            NetworkIdenticon(
                identicon: viewModel.identicon,
                network: viewModel.network,
                background: Asset.backgroundPrimary.swiftUIColor,
                size: Heights.identiconInCell
            )
            if viewModel.path.isEmpty, !viewModel.hasPassword {
                Localizable.BackupModal.Label.emptyPath.text
                    .foregroundColor(Asset.textAndIconsDisabled.swiftUIColor)
                    .font(PrimaryFont.captionM.font)
                    .frame(idealWidth: .infinity, maxWidth: .infinity, alignment: .leading)
            } else {
                fullPath
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(PrimaryFont.bodyL.font)
                    .frame(idealWidth: .infinity, maxWidth: .infinity, alignment: .leading)
            }
            NetworkCapsuleView(network: viewModel.network)
        }
        .padding([.top, .bottom], Spacing.extraSmall)
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
}

#if DEBUG
    struct DerivedKeyOverviewRow_Previews: PreviewProvider {
        static var previews: some View {
            VStack(alignment: .leading) {
                DerivedKeyOverviewRow(
                    DerivedKeyOverviewViewModel(
                        identicon: PreviewData.exampleIdenticon,
                        path: "",
                        hasPassword: false,
                        network: "Kusama"
                    )
                )
                DerivedKeyOverviewRow(
                    DerivedKeyOverviewViewModel(
                        identicon: PreviewData.exampleIdenticon,
                        path: "//polkadot",
                        hasPassword: false,
                        network: "Polkadot"
                    )
                )
                DerivedKeyOverviewRow(
                    DerivedKeyOverviewViewModel(
                        identicon: PreviewData.exampleIdenticon,
                        path: "//astar",
                        hasPassword: false,
                        network: "Astar"
                    )
                )
                DerivedKeyOverviewRow(
                    DerivedKeyOverviewViewModel(
                        identicon: PreviewData.exampleIdenticon,
                        path: "//kusama",
                        hasPassword: true,
                        network: "Kusama"
                    )
                )
                DerivedKeyOverviewRow(
                    DerivedKeyOverviewViewModel(
                        identicon: PreviewData.exampleIdenticon,
                        path: "//kusama//verylongpathsolongthatmightbemultilineandhaspasswordtoo",
                        hasPassword: true,
                        network: "Kusama"
                    )
                )
            }
            .preferredColorScheme(.dark)
        }
    }
#endif
