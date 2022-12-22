//
//  LogsListView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 02/12/2022.
//

import SwiftUI

struct LogsListView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.LogsList.Label.title.string,
                    leftButton: .empty,
                    rightButton: .more,
                    backgroundColor: Asset.backgroundSystem.swiftUIColor
                ),
                actionModel: .init(
                    rightBarMenuAction: viewModel.onMoreMenuTap
                )
            )
            ScrollView {
                LazyVStack(alignment: .leading, spacing: 0) {
                    ForEach(viewModel.renderables, id: \.id) { renderable in
                        LogEntryView(viewModel: .init(renderable: renderable))
                    }
                }
            }
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
        .onAppear {
            viewModel.use(navigation: navigation)
            viewModel.loadData()
        }
        .fullScreenCover(
            isPresented: $viewModel.isShowingActionSheet,
            onDismiss: { viewModel.onMoreActionSheetDismissal() }
        ) {
            LogsMoreActionsModal(
                isShowingActionSheet: $viewModel.isShowingActionSheet,
                shouldPresentClearConfirmationModal: $viewModel.shouldPresentClearConfirmationModal,
                shouldPresentAddNoteModal: $viewModel.shouldPresentAddNoteModal
            )
            .clearModalBackground()
        }
        .fullScreenCover(isPresented: $viewModel.isPresentingClearConfirmationModal) {
            HorizontalActionsBottomModal(
                viewModel: .clearLog,
                mainAction: viewModel.clearLogsAction(),
                isShowingBottomAlert: $viewModel.isPresentingClearConfirmationModal
            )
            .clearModalBackground()
        }
    }
}

extension LogsListView {
    final class ViewModel: ObservableObject {
        @Published var logs: MLog
        @Published var renderables: [LogEntryRenderable] = []
        @Published var shouldPresentClearConfirmationModal = false
        @Published var shouldPresentAddNoteModal = false
        @Published var isShowingActionSheet = false
        @Published var isPresentingClearConfirmationModal = false

        private weak var navigation: NavigationCoordinator!

        init(
            logs: MLog
        ) {
            self.logs = logs
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func loadData() {
            renderables = LogEntryRenderableBuilder().build(logs)
        }

        func onMoreMenuTap() {
            navigation.performFake(navigation: .init(action: .rightButtonAction))
            isShowingActionSheet = true
        }

        func onMoreActionSheetDismissal() {
            if shouldPresentAddNoteModal {
                shouldPresentAddNoteModal = false
                navigation.perform(navigation: .init(action: .createLogComment))
            }
            if shouldPresentClearConfirmationModal {
                shouldPresentClearConfirmationModal = false
                isPresentingClearConfirmationModal = true
            }
        }

        func clearLogsAction() {
            if case let .log(updatedLogs) = navigation.performFake(navigation: .init(action: .clearLog)).screenData {
                logs = updatedLogs
                loadData()
            }
        }
    }
}

#if DEBUG
    struct LogsListView_Previews: PreviewProvider {
        static var previews: some View {
            LogsListView(viewModel: .init(logs: MLog(log: [History(
                order: 0,
                timestamp: "43254353453",
                events: [.databaseInitiated, .deviceWasOnline, .historyCleared, .identitiesWiped]
            )])))
            .environmentObject(NavigationCoordinator())
        }
    }
#endif
