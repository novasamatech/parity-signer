//
//  KeySetList.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 22/08/2022.
//

import SwiftUI

struct KeySetList: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject var appState: AppState
    @State private var isShowingNewSeedMenu = false
    @State private var isShowingCreateKeySet = false
    @State private var isShowingMoreMenu = false
    @State private var isExportKeysSelected = false
    @State private var shouldShowCreateKeySet = false

    @State var selectedItems: [KeySetViewModel] = []

    var body: some View {
        ZStack(alignment: .bottom) {
            // Main screen
            VStack(spacing: 0) {
                // Navigation Bar
                NavigationBarView(
                    viewModel: NavigationBarViewModel(
                        title: Localizable.KeySets.title.string,
                        leftButtons: [.init(
                            type: isExportKeysSelected ? .xmark : .empty,
                            action: {
                                isExportKeysSelected.toggle()
                                selectedItems.removeAll()
                            }
                        )],
                        rightButtons: [.init(
                            type: isExportKeysSelected ? .empty : .more,
                            action: {
                                isShowingMoreMenu.toggle()
                            }
                        )],
                        backgroundColor: Asset.backgroundSystem.swiftUIColor
                    )
                )
                // Empty state
                if viewModel.listViewModel.list.isEmpty {
                    KeyListEmptyState()
                } else {
                    // List of Key Sets
                    List {
                        ForEach(
                            viewModel.listViewModel.list.sorted(by: { $0.keyName < $1.keyName }),
                            id: \.keyName
                        ) { keyItem($0) }
                        Spacer()
                            .listRowBackground(Asset.backgroundSystem.swiftUIColor)
                            .listRowSeparator(.hidden)
                            .frame(height: Heights.actionButton + Spacing.large + Heights.tabbarHeight)
                    }
                    .listStyle(.plain)
                    .hiddenScrollContent()
                }
            }
            .background(
                Asset.backgroundSystem.swiftUIColor
                    .ignoresSafeArea()
            )
            VStack {
                // Add Key Set
                if !isExportKeysSelected {
                    VStack(spacing: 0) {
                        ConnectivityAlertOverlay(viewModel: .init())
                        PrimaryButton(
                            action: {
                                // We need to call this conditionally, as if there are no seeds,
                                // Rust does not expect `rightButtonAction` called before `addSeed` / `recoverSeed`
                                if !viewModel.listViewModel.list.isEmpty {
                                    navigation.performFake(navigation: .init(action: .rightButtonAction))
                                }
                                isShowingNewSeedMenu.toggle()
                            },
                            text: Localizable.KeySets.Action.add.key
                        )
                        .padding(.horizontal, Spacing.large)
                        .padding(.bottom, Spacing.large)
                    }
                }
                TabBarView(
                    selectedTab: $navigation.selectedTab
                )
            }
            if isExportKeysSelected {
                exportKeysOverlay
            }
        }
        .fullScreenCover(
            isPresented: $isShowingNewSeedMenu,
            onDismiss: {
                // iOS 15 handling of following .fullscreen presentation after dismissal, we need to dispatch this async
                DispatchQueue.main.async {
                    if shouldShowCreateKeySet {
                        shouldShowCreateKeySet = false
                        isShowingCreateKeySet = true
                    }
                }
            }
        ) {
            AddKeySetModal(
                isShowingNewSeedMenu: $isShowingNewSeedMenu,
                shouldShowCreateKeySet: $shouldShowCreateKeySet
            )
            .clearModalBackground()
        }
        .fullScreenCover(
            isPresented: $isShowingCreateKeySet,
            onDismiss: viewModel.updateData
        ) {
            EnterKeySetNameView(viewModel: .init(isPresented: $isShowingCreateKeySet))
        }
        .fullScreenCover(isPresented: $isShowingMoreMenu) {
            KeyListMoreMenuModal(
                isPresented: $isShowingMoreMenu,
                isExportKeysSelected: $isExportKeysSelected
            )
            .clearModalBackground()
        }
        .fullScreenCover(
            isPresented: $viewModel.isShowingKeysExportModal
        ) {
            ExportMultipleKeysModal(
                viewModel: .init(
                    viewModel: ExportMultipleKeysModalViewModel(
                        selectedItems: .keySets(selectedItems),
                        count: selectedItems.count
                    ),
                    isPresented: $viewModel.isShowingKeysExportModal
                )
            )
            .clearModalBackground()
            .onAppear {
                selectedItems.removeAll()
                isExportKeysSelected.toggle()
            }
        }
    }

    func keyItem(_ viewModel: KeySetViewModel) -> some View {
        KeySetRow(
            viewModel: viewModel,
            selectedItems: $selectedItems,
            isExportKeysSelected: $isExportKeysSelected
        )
        .onTapGesture {
            if isExportKeysSelected {
                if selectedItems.contains(viewModel) {
                    selectedItems.removeAll { $0 == viewModel }
                } else {
                    selectedItems.append(viewModel)
                }
            } else {
                self.viewModel.loadKeysInformation(for: viewModel.keyName) { result in
                    switch result {
                    case let .success(keysData):
                        appState.userData.keysData = keysData
                        navigation.perform(navigation: .init(action: .selectSeed, details: viewModel.keyName))
                    case .failure:
                        ()
                    }
                }
            }
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

    var exportKeysOverlay: some View {
        HStack {
            Button(action: {
                selectedItems = viewModel.listViewModel.list
                viewModel.isShowingKeysExportModal.toggle()
            }) {
                Localizable.KeySets.More.Action.exportAll.text
                    .foregroundColor(Asset.accentPink300.swiftUIColor)
                    .font(PrimaryFont.labelL.font)
            }
            .padding(.leading, Spacing.medium)
            Spacer()
            Button(action: {
                viewModel.isShowingKeysExportModal.toggle()
            }) {
                Localizable.KeySets.More.Action.exportSelected.text
                    .foregroundColor(
                        selectedItems.isEmpty ?
                            Asset.textAndIconsDisabled.swiftUIColor :
                            Asset.accentPink300
                            .swiftUIColor
                    )
                    .font(PrimaryFont.labelL.font)
            }
            .disabled(selectedItems.isEmpty)
            .padding(.trailing, Spacing.medium)
        }
        .frame(height: Heights.tabbarHeight)
        .background(Asset.backgroundSecondary.swiftUIColor)
    }
}

extension KeySetList {
    final class ViewModel: ObservableObject {
        private let keyListService: KeyListService
        let keyDetailsService: KeyDetailsService
        private let modelBuilder: KeySetListViewModelBuilder
        @Published var isShowingKeysExportModal = false
        @Published var listViewModel: KeySetListViewModel = .init(list: [])
        private var dataModel: MSeeds

        init(
            keyDetailsService: KeyDetailsService = KeyDetailsService(),
            keyListService: KeyListService = KeyListService(),
            modelBuilder: KeySetListViewModelBuilder = KeySetListViewModelBuilder(),
            dataModel: MSeeds
        ) {
            self.keyDetailsService = keyDetailsService
            self.keyListService = keyListService
            self.modelBuilder = modelBuilder
            self.dataModel = dataModel
            updateView(dataModel)
        }

        func updateView(_ dataModel: MSeeds) {
            listViewModel = modelBuilder.build(for: dataModel)
        }

        func updateData() {
            dataModel = keyListService.getKeyList()
            updateView(dataModel)
        }

        func loadKeysInformation(
            for seedName: String,
            _ completion: @escaping (Result<MKeysNew, ServiceError>) -> Void
        ) {
            keyDetailsService.getKeys(for: seedName, completion)
        }
    }
}

private struct KeyListEmptyState: View {
    var body: some View {
        VStack(spacing: Spacing.extraSmall) {
            Spacer()
            Text(Localizable.KeySets.Label.Empty.title.key)
                .font(PrimaryFont.titleM.font)
            Text(Localizable.KeySets.Label.Empty.subtitle.key)
                .font(PrimaryFont.bodyL.font)
            Spacer()
                .frame(height: Heights.actionButton + 2 * Spacing.large)
            Spacer()
        }
        .padding(Spacing.componentSpacer)
        .multilineTextAlignment(.center)
        .lineSpacing(Spacing.extraExtraSmall)
        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
    }
}

#if DEBUG
    struct KeySetListPreview: PreviewProvider {
        static var previews: some View {
            KeySetList(
                viewModel: .init(dataModel: PreviewData.mseeds)
            )
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
            .environmentObject(NavigationCoordinator())
            .environmentObject(AppState.preview)
        }
    }
#endif
