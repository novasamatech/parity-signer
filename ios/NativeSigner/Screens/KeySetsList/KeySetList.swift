//
//  KeySetList.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 22/08/2022.
//

import SwiftUI

struct KeySetList: View {
    @ObservedObject private var navigation: NavigationCoordinator
    @State private var isShowingNewSeedMenu = false

    private let viewModel: KeySetListViewModel

    init(
        navigation: NavigationCoordinator,
        viewModel: KeySetListViewModel
    ) {
        self.navigation = navigation
        self.viewModel = viewModel
    }

    var body: some View {
        ZStack(alignment: .bottom) {
            VStack(spacing: 0) {
                NavigationBarView(
                    navigation: navigation,
                    viewModel: NavigationBarViewModel(title: Localizable.KeySets.title.string)
                )
                List {
                    ForEach(
                        viewModel.list.sorted(by: { $0.keyName < $1.keyName }),
                        id: \.keyName
                    ) { keySet in
                        KeySetRow(keySet).onTapGesture {
                            navigation.perform(navigation: .init(action: .selectSeed, details: keySet.keyName))
                        }
                        .listRowBackground(Asset.backgroundSystem.swiftUIColor)
                        .listRowSeparator(.hidden)
                        .listRowInsets(.init(
                            top: Spacing.extraExtraSmall,
                            leading: Spacing.extraSmall,
                            bottom: Spacing.extraExtraSmall,
                            trailing: Spacing.extraSmall
                        ))
                    }
                    Spacer()
                        .listRowBackground(Asset.backgroundSystem.swiftUIColor)
                        .listRowSeparator(.hidden)
                        .frame(height: Heights.actionButton + Spacing.large)
                }
                .listStyle(.plain)
            }
            PrimaryButton(
                action: {
                    // We need to call this conditionally, as if there are no seeds,
                    // Rust does not expect `rightButtonAction` called before `addSeed` / `recoverSeed`
                    if !viewModel.list.isEmpty {
                        navigation.perform(navigation: .init(action: .rightButtonAction))
                    }
                    isShowingNewSeedMenu.toggle()
                },
                text: Localizable.KeySets.Action.add.key
            )
            .padding(Spacing.large)
        }
        .fullScreenCover(isPresented: $isShowingNewSeedMenu) {
            AddKeySetModal(
                isShowingNewSeedMenu: $isShowingNewSeedMenu,
                navigation: navigation
            )
            .clearModalBackground()
        }
    }
}

// struct KeySetListPreview: PreviewProvider {
//    static var previews: some View {
//        KeySetList(
//            navigation: NavigationCoordinator(),
//            viewModel: KeySetListViewModelBuilder()
//                .build(
//                    for:
//                    MSeeds(
//                        seedNameCards: [
//                            SeedNameCard(
//                                seedName: "aaaa",
//                                identicon: PreviewData.exampleIdenticon,
//                                derivedKeysCount: 3
//                            ),
//                            SeedNameCard(
//                                seedName: "bbbb",
//                                identicon: PreviewData.exampleIdenticon,
//                                derivedKeysCount: 0
//                            ),
//                            SeedNameCard(
//                                seedName: "cccc",
//                                identicon: PreviewData.exampleIdenticon,
//                                derivedKeysCount: 1
//                            ),
//                            SeedNameCard(
//                                seedName: "dddd",
//                                identicon: PreviewData.exampleIdenticon,
//                                derivedKeysCount: 4
//                            ),
//                            SeedNameCard(
//                                seedName: "eeee",
//                                identicon: PreviewData.exampleIdenticon,
//                                derivedKeysCount: 15
//                            ),
//                            SeedNameCard(
//                                seedName: "ffff",
//                                identicon: PreviewData.exampleIdenticon,
//                                derivedKeysCount: 1
//                            ),
//                            SeedNameCard(
//                                seedName: "gggg",
//                                identicon: PreviewData.exampleIdenticon,
//                                derivedKeysCount: 0
//                            )
//                        ]
//                    )
//                )
//        )
//        .preferredColorScheme(.dark)
//        .previewLayout(.sizeThatFits)
//    }
// }
