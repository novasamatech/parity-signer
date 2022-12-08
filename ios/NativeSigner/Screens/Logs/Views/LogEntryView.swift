//
//  LogEntryView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 07/12/2022.
//

import SwiftUI

struct LogEntryView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            if let date = viewModel.renderable.dateHeader {
                HStack(alignment: .center) {
                    Text(date)
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                        .font(Fontstyle.bodyM.base)
                }
                .frame(height: Heights.dateHeaderHeight)
            }
            HStack {
                HStack(alignment: .bottom, spacing: Spacing.extraSmall) {
                    VStack(alignment: .leading, spacing: 0) {
                        Text(viewModel.renderable.title)
                            .foregroundColor(
                                viewModel.renderable.isWarning ?
                                    Asset.accentRed300.swiftUIColor :
                                    Asset.textAndIconsPrimary.swiftUIColor
                            )
                            .font(Fontstyle.titleS.base)
                        if let displayValue = viewModel.renderable.displayValue {
                            Text(displayValue)
                                .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                                .font(Fontstyle.bodyM.base)
                                .padding(.top, Spacing.small)
                        }
                        if let additionalValue = viewModel.renderable.additionalValue {
                            Text(additionalValue)
                                .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                                .font(Fontstyle.captionM.base)
                                .padding(.top, Spacing.extraExtraSmall)
                        }
                    }
                    Spacer()
                    VStack(alignment: .trailing) {
                        if viewModel.renderable.type != .basic {
                            ZStack {
                                Circle()
                                    .frame(
                                        width: Sizes.chevronCircleButton,
                                        height: Sizes.chevronCircleButton,
                                        alignment: .center
                                    )
                                    .foregroundColor(Asset.fill6.swiftUIColor)
                                Asset.chevronRight.swiftUIImage
                                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                            }
                            Spacer()
                        }
                        HStack(spacing: Spacing.extraSmall) {
                            Text(DateFormatter.hourMinutes(viewModel.renderable.timestamp))
                        }
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    }
                }
            }
            .contentShape(Rectangle())
            .onTapGesture {
                viewModel.onEventTap()
            }
            .padding(.vertical, Spacing.small)
        }
        .padding(.horizontal, Spacing.large)
        .onAppear {
            viewModel.use(navigation: navigation)
        }
    }
}

extension LogEntryView {
    final class ViewModel: ObservableObject {
        let renderable: LogEntryRenderable

        private weak var navigation: NavigationCoordinator!

        init(renderable: LogEntryRenderable) {
            self.renderable = renderable
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func onEventTap() {
            guard renderable.type != .basic else { return }
            navigation.perform(
                navigation: .init(
                    action: .showLogDetails,
                    details: renderable.navigationDetails
                )
            )
        }
    }
}

//
// #if DEBUG
//    struct LogEntryView_Previews: PreviewProvider {
//        static var previews: some View {
//            LogsListView(viewModel: .init(logs: MLog(log: [History(
//                order: 0,
//                timestamp: "43254353453",
//                events: [.databaseInitiated, .deviceWasOnline, .historyCleared, .identitiesWiped]
//            )])))
//            .environmentObject(NavigationCoordinator())
//        }
//    }
// #endif
