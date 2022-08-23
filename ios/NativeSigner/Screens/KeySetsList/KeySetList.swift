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
                NavigationBarView(title: Localizable.KeySets.title.string)
                List {
                    ForEach(
                        viewModel.list.sorted(by: { $0.keyName < $1.keyName }),
                        id: \.keyName
                    ) { keySet in
                        KeySetRow(keySet).onTapGesture {
                            navigation.perform(navigation: .init(action: .selectSeed, details: keySet.keyName))
                        }
                        .listRowSeparator(.hidden)
                        .listRowInsets(.init(
                            top: Padding.extraExtraSmall,
                            leading: Padding.extraSmall,
                            bottom: Padding.extraExtraSmall,
                            trailing: Padding.extraSmall
                        ))
                    }
                    Spacer()
                        .listRowSeparator(.hidden)
                        .frame(height: Heights.actionButton + Padding.large)
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
            .padding(Padding.large)
        }
        .background(Asset.backgroundSolidSystem.swiftUIColor)
        .sheet(isPresented: $isShowingNewSeedMenu) {
            NewSeedMenu(isShowingNewSeedMenu: $isShowingNewSeedMenu, navigation: navigation)
        }
    }
}

struct KeySetListPreview: PreviewProvider {
    static var previews: some View {
        KeySetList(
            navigation: NavigationCoordinator(),
            viewModel: KeySetListViewModelBuilder()
                .build(
                    for:
                    MSeeds(
                        seedNameCards: [
                            SeedNameCard(seedName: "aaaa", identicon: PreviewData.exampleIdenticon),
                            SeedNameCard(seedName: "bbbb", identicon: PreviewData.exampleIdenticon),
                            SeedNameCard(seedName: "cccc", identicon: PreviewData.exampleIdenticon),
                            SeedNameCard(seedName: "dddd", identicon: PreviewData.exampleIdenticon),
                            SeedNameCard(seedName: "eeee", identicon: PreviewData.exampleIdenticon),
                            SeedNameCard(seedName: "ffff", identicon: PreviewData.exampleIdenticon),
                            SeedNameCard(seedName: "gggg", identicon: PreviewData.exampleIdenticon)
                        ]
                    )
                )
        )
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
    }
}
