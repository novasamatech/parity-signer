//
//  DerivedKeyRow.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 30/08/2022.
//

import SwiftUI

struct DerivedKeyRow: View {
    let viewModel: DerivedKeyRowViewModel
    @Binding var selectedKeys: [DerivedKeyRowModel]
    @Binding var isPresentingSelectionOverlay: Bool

    private var isItemSelected: Bool {
        selectedKeys.map(\.viewModel).contains(viewModel)
    }

    var body: some View {
        HStack(alignment: .center, spacing: Spacing.small) {
            Identicon(identicon: viewModel.identicon, rowHeight: Heights.identiconInCell)
                .padding(.top, Spacing.extraExtraSmall)
                .padding(.leading, Spacing.medium)
            VStack(alignment: .leading) {
                if !isRoot {
                    fullPath
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                        .font(PrimaryFont.captionM.font)
                    Spacer().frame(height: Spacing.extraExtraSmall)
                }
                HStack(spacing: Spacing.extraExtraSmall) {
                    Text(viewModel.base58.truncateMiddle())
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.bodyL.font)
                        .lineLimit(1)
                }
            }
            Spacer()
            VStack(alignment: .center) {
                if isPresentingSelectionOverlay {
                    if isItemSelected {
                        Asset.checkmarkChecked.swiftUIImage
                    } else {
                        Asset.checkmarkUnchecked.swiftUIImage
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    }
                } else {
                    Asset.chevronRight.swiftUIImage
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                }
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
                        identicon: PreviewData.exampleIdenticon,
                        path: "// polkadot",
                        hasPassword: false,
                        base58: "15Gsc678654FDSG0HA04H0A",
                        rootKeyName: ""
                    ),
                    selectedKeys: Binding<[DerivedKeyRowModel]>.constant([]),
                    isPresentingSelectionOverlay: Binding<Bool>.constant(true)
                )
                DerivedKeyRow(
                    viewModel: DerivedKeyRowViewModel(
                        identicon: PreviewData.exampleIdenticon,
                        path: "",
                        hasPassword: false,
                        base58: "15Gsc678654FDSG0HA04H0A"
                    ),
                    selectedKeys: Binding<[DerivedKeyRowModel]>.constant([]),
                    isPresentingSelectionOverlay: Binding<Bool>.constant(true)
                )
                DerivedKeyRow(
                    viewModel: DerivedKeyRowViewModel(
                        identicon: PreviewData.exampleIdenticon,
                        path: "// astar",
                        hasPassword: false,
                        base58: "15Gsc678654FDSG0HA04H0A"
                    ),
                    selectedKeys: Binding<[DerivedKeyRowModel]>.constant([]),
                    isPresentingSelectionOverlay: Binding<Bool>.constant(true)
                )
                DerivedKeyRow(
                    viewModel: DerivedKeyRowViewModel(
                        identicon: PreviewData.exampleIdenticon,
                        path: "// kusama",
                        hasPassword: true,
                        base58: "15Gsc678654FDSG0HA04H0A"
                    ),
                    selectedKeys: Binding<[DerivedKeyRowModel]>.constant([]),
                    isPresentingSelectionOverlay: Binding<Bool>.constant(false)
                )
                DerivedKeyRow(
                    viewModel: DerivedKeyRowViewModel(
                        identicon: PreviewData.exampleIdenticon,
                        path: "// kusama // verylongpathsolongitrequirestwolinesoftextormaybeevenmoremaybethree",
                        hasPassword: true,
                        base58: "15Gsc678654FDSG0HA04H0A"
                    ),
                    selectedKeys: Binding<[DerivedKeyRowModel]>.constant([]),
                    isPresentingSelectionOverlay: Binding<Bool>.constant(false)
                )
            }
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
        }
    }
#endif
