//
//  DerivedKeyOverviewRow.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 19/09/2022.
//

import SwiftUI

struct DerivedKeyOverviewViewModel: Equatable {
    let identicon: [UInt8]
    let path: String
    let hasPassword: Bool

    init(
        identicon: [UInt8],
        path: String,
        hasPassword: Bool
    ) {
        self.identicon = identicon
        self.path = path
        self.hasPassword = hasPassword
    }
}

extension DerivedKeyOverviewViewModel {
    init(_ key: MKeyAndNetworkCard) {
        path = key.key.address.path
        identicon = key.key.address.identicon.svgPayload
        hasPassword = key.key.address.hasPwd
    }
}

struct DerivedKeyOverviewRow: View {
    private let viewModel: DerivedKeyOverviewViewModel

    init(_ viewModel: DerivedKeyOverviewViewModel) {
        self.viewModel = viewModel
    }

    var body: some View {
        HStack(alignment: .center, spacing: Spacing.small) {
            Identicon(identicon: viewModel.identicon, rowHeight: Heights.identiconInCell)
            fullPath
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.titleS.font)
                .frame(idealWidth: .infinity, maxWidth: .infinity, alignment: .leading)
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

struct DerivedKeyOverviewRow_Previews: PreviewProvider {
    static var previews: some View {
        VStack(alignment: .leading) {
            DerivedKeyOverviewRow(
                DerivedKeyOverviewViewModel(
                    identicon: PreviewData.exampleIdenticon,
                    path: "// polkadot",
                    hasPassword: false
                )
            )
            DerivedKeyOverviewRow(
                DerivedKeyOverviewViewModel(
                    identicon: PreviewData.exampleIdenticon,
                    path: "// astar",
                    hasPassword: false
                )
            )
            DerivedKeyOverviewRow(
                DerivedKeyOverviewViewModel(
                    identicon: PreviewData.exampleIdenticon,
                    path: "// kusama",
                    hasPassword: true
                )
            )
            DerivedKeyOverviewRow(
                DerivedKeyOverviewViewModel(
                    identicon: PreviewData.exampleIdenticon,
                    path: "// kusama // verylongpathsolongthatmightbemultilineandhaspasswordtoo",
                    hasPassword: true
                )
            )
        }
        .preferredColorScheme(.dark)
//        .previewLayout(.sizeThatFits)
    }
}
