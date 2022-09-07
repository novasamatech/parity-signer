//
//  KeyDetailsView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 29/08/2022.
//

import SwiftUI

struct KeyDetailsView: View {
    @State private var isShowingActionSheet = false
    @ObservedObject private var navigation: NavigationCoordinator
    private var seedsMediator: SeedsMediating!

    private let alertClosure: (() -> Void)?
    private let viewModel: KeyDetailsViewModel
    private let actionModel: KeyDetailsActionModel

    init(
        navigation: NavigationCoordinator,
        seedsMediator: SeedsMediating? = nil,
        viewModel: KeyDetailsViewModel,
        actionModel: KeyDetailsActionModel,
        alertClosure: (() -> Void)? = nil
    ) {
        self.navigation = navigation
        self.seedsMediator = seedsMediator
        self.viewModel = viewModel
        self.actionModel = actionModel
        self.alertClosure = alertClosure
    }

    var body: some View {
        ZStack(alignment: .bottom) {
            VStack(spacing: 0) {
                // Navigation bar
                NavigationBarView(
                    navigation: navigation,
                    viewModel: .init(
                        isBackButtonVisible: true,
                        isRightBarMenuButtonVisible: true
                    ),
                    actionModel: .init(rightBarMenuAction: {
                        isShowingActionSheet.toggle()
                    })
                )
                // Main key cell
                HStack {
                    VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                        Text(viewModel.keyName)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            .font(Fontstyle.titleL.base)
                        Text(viewModel.base58)
                            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                            .font(Fontstyle.bodyM.base)
                            .lineLimit(1)
                    }
                    Spacer().frame(maxWidth: .infinity)
                    Asset.chevronRight.swiftUIImage
                }
                .padding(Padding.detailsCell)
                .onTapGesture {
                    navigation.perform(navigation: actionModel.addressKeyNavigation)
                }
                // Header
                HStack {
                    Localizable.KeyDetails.Label.derived.text
                        .font(Fontstyle.bodyM.base)
                    Spacer().frame(maxWidth: .infinity)
                    Button(
                        action: {
                            navigation.perform(navigation: .init(action: .networkSelector))
                        }, label: {
                            Asset.switches.swiftUIImage
                        }
                    )
                }
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                .padding(Padding.detailsCell)
                // List of derived keys
                List {
                    ForEach(
                        viewModel.derivedKeys,
                        id: \.viewModel.path
                    ) { deriveKey in
                        DerivedKeyRow(deriveKey.viewModel)
                            .listRowBackground(Asset.backgroundSystem.swiftUIColor)
                            .listRowSeparator(.hidden)
                            .listRowInsets(EdgeInsets())
                            .onTapGesture {
                                navigation.perform(navigation: deriveKey.actionModel.tapAction)
                            }
                    }
                    Spacer()
                        .listRowBackground(Asset.backgroundSystem.swiftUIColor)
                        .listRowSeparator(.hidden)
                        .frame(height: Heights.actionButton + Spacing.large)
                }
                .listStyle(.plain)
            }
            .background(Asset.backgroundSystem.swiftUIColor)
            // Main CTA
            PrimaryButton(
                action: {
                    if let alertClosure = actionModel.alertClosure {
                        alertClosure()
                    } else {
                        navigation.perform(navigation: actionModel.createDerivedKey)
                    }
                },
                text: Localizable.KeyDetails.Action.create.key
            )
            .padding(Spacing.large)
        }
        .fullScreenCover(isPresented: $isShowingActionSheet) {
            KeyDetailsActionsModal(
                isShowingActionSheet: $isShowingActionSheet,
                navigation: navigation,
                removeSeed: {
                    seedsMediator?.removeSeed(seedName: actionModel.removeSeed)
                }
            )
            .clearModalBackground()
        }
    }
}

//
// struct KeyDetailsView_Previews: PreviewProvider {
//    static var previews: some View {
//        VStack {
//            KeyDetailsView(
//                navigation: NavigationCoordinator(),
//                viewModel: .init(
//                    keyName: "Parity",
//                    base58: "15Gsc678...0HA04H0A",
//                    derivedKeys: [
//                        DerivedKeyRowModel(
//                            viewModel: DerivedKeyRowViewModel(
//                                identicon: PreviewData.exampleIdenticon,
//                                path: "// polkadot",
//                                hasPassword: false,
//                                base58: "15Gsc678654FDSG0HA04H0A"
//                            ),
//                            actionModel: DerivedKeyActionModel(
//                                tapAction: .init(action: .rightButtonAction)
//                            )
//                        ),
//
//                        DerivedKeyRowModel(
//                            viewModel: DerivedKeyRowViewModel(
//                                identicon: PreviewData.exampleIdenticon,
//                                path: "// polkadot",
//                                hasPassword: false,
//                                base58: "15Gsc678654FDSG0HA04H0A"
//                            ),
//                            actionModel: DerivedKeyActionModel(
//                                tapAction: .init(action: .rightButtonAction)
//                            )
//                        ),
//                        DerivedKeyRowModel(
//                            viewModel: DerivedKeyRowViewModel(
//                                identicon: PreviewData.exampleIdenticon,
//                                path: "// astar",
//                                hasPassword: false,
//                                base58: "15Gsc678654FDSG0HA04H0A"
//                            ),
//                            actionModel: DerivedKeyActionModel(
//                                tapAction: .init(action: .rightButtonAction)
//                            )
//                        ),
//                        DerivedKeyRowModel(
//                            viewModel: DerivedKeyRowViewModel(
//                                identicon: PreviewData.exampleIdenticon,
//                                path: "// verylongpathsolongitrequirestwolinesoftextormaybeevenmoremaybethree",
//                                hasPassword: false,
//                                base58: "15Gsc678654FDSG0HA04H0A"
//                            ),
//                            actionModel: DerivedKeyActionModel(
//                                tapAction: .init(action: .rightButtonAction)
//                            )
//                        ),
//                        DerivedKeyRowModel(
//                            viewModel: DerivedKeyRowViewModel(
//                                identicon: PreviewData.exampleIdenticon,
//                                path: "// acala",
//                                hasPassword: true,
//                                base58: "15Gsc678654FDSG0HA04H0A"
//                            ),
//                            actionModel: DerivedKeyActionModel(
//                                tapAction: .init(action: .rightButtonAction)
//                            )
//                        ),
//                        DerivedKeyRowModel(
//                            viewModel: DerivedKeyRowViewModel(
//                                identicon: PreviewData.exampleIdenticon,
//                                path: "// moonbeam",
//                                hasPassword: true,
//                                base58: "15Gsc678654FDSG0HA04H0A"
//                            ),
//                            actionModel: DerivedKeyActionModel(
//                                tapAction: .init(action: .rightButtonAction)
//                            )
//                        ),
//                        DerivedKeyRowModel(
//                            viewModel: DerivedKeyRowViewModel(
//                                identicon: PreviewData.exampleIdenticon,
//                                path: "// kilt",
//                                hasPassword: true,
//                                base58: "15Gsc6786546423FDSG0HA04H0A"
//                            ),
//                            actionModel: DerivedKeyActionModel(
//                                tapAction: .init(action: .rightButtonAction)
//                            )
//                        )
//                    ]
//                ),
//                actionModel: KeyDetailsActionModel(
//                    addressKeyNavigation: .init(action: .goBack),
//                    derivedKeysNavigation: [],
//                    alertClosure: nil
//                )
//            )
//        }
//        .preferredColorScheme(.dark)
//        .previewLayout(.sizeThatFits)
//    }
// }
