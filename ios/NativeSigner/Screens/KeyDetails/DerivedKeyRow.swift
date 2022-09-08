//
//  DerivedKeyRow.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 30/08/2022.
//

import SwiftUI

struct DerivedKeyRow: View {
    private let viewModel: DerivedKeyRowViewModel

    init(_ viewModel: DerivedKeyRowViewModel) {
        self.viewModel = viewModel
    }

    var body: some View {
        HStack(alignment: .top, spacing: Spacing.small) {
            VStack {
                Identicon(identicon: viewModel.identicon, rowHeight: Heights.identiconInCell)
                    .padding(.top, Spacing.extraExtraSmall)
            }
            VStack(alignment: .leading) {
                HStack(alignment: .top, spacing: 0) {
                    Text(viewModel.path)
                    if viewModel.hasPassword {
                        Localizable.Path.delimeter.text
                        Image(.lock)
                    }
                }
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(Fontstyle.titleS.base)
                Spacer().frame(height: Spacing.extraExtraSmall)
                HStack(spacing: Spacing.extraExtraSmall) {
                    Asset.derivedKeyAddress.swiftUIImage
                    Text(viewModel.base58)
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                        .font(Fontstyle.bodyM.base)
                        .lineLimit(1)
                }
            }
            Spacer()
            VStack(alignment: .center) {
                Asset.chevronRight.swiftUIImage
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            }
            .frame(minHeight: .zero, maxHeight: .infinity)
        }
        .padding(Spacing.medium)
        .fixedSize(horizontal: false, vertical: true)
    }
}

// struct DerivedKeyRow_Previews: PreviewProvider {
//    static var previews: some View {
//        VStack {
//            DerivedKeyRow(
//                DerivedKeyRowViewModel(
//                    identicon: PreviewData.exampleIdenticon,
//                    path: "// polkadot",
//                    hasPassword: false,
//                    base58: "15Gsc678654FDSG0HA04H0A"
//                )
//            )
//            DerivedKeyRow(
//                DerivedKeyRowViewModel(
//                    identicon: PreviewData.exampleIdenticon,
//                    path: "// astar",
//                    hasPassword: false,
//                    base58: "15Gsc678654FDSG0HA04H0A"
//                )
//            )
//            DerivedKeyRow(
//                DerivedKeyRowViewModel(
//                    identicon: PreviewData.exampleIdenticon,
//                    path: "// kusama",
//                    hasPassword: true,
//                    base58: "15Gsc678654FDSG0HA04H0A"
//                )
//            )
//            DerivedKeyRow(
//                DerivedKeyRowViewModel(
//                    identicon: PreviewData.exampleIdenticon,
//                    path: "// kusama // verylongpathsolongitrequirestwolinesoftextormaybeevenmoremaybethree",
//                    hasPassword: true,
//                    base58: "15Gsc678654FDSG0HA04H0A"
//                )
//            )
//        }
//        .preferredColorScheme(.dark)
//        .previewLayout(.sizeThatFits)
//    }
// }
