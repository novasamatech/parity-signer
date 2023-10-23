//
//  LogEntryView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 07/12/2022.
//

import SwiftUI

struct LogEntryView: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            if let date = viewModel.renderable.dateHeader {
                HStack(alignment: .center) {
                    Text(date)
                        .foregroundColor(.textAndIconsTertiary)
                        .font(PrimaryFont.bodyM.font)
                        .padding(.vertical, Spacing.small)
                }
            }
            HStack {
                VStack(alignment: .leading, spacing: 0) {
                    HStack(alignment: .top, spacing: Spacing.extraSmall) {
                        Text(viewModel.renderable.title)
                            .foregroundColor(
                                viewModel.renderable.isWarning ?
                                    .accentRed300 :
                                    .textAndIconsPrimary
                            )
                            .font(PrimaryFont.titleS.font)
                        Spacer()
                        HStack(spacing: 0) {
                            Text(viewModel.renderable.timestamp)
                                .padding(.leading, Spacing.extraSmall)
                            if viewModel.renderable.type != .basic {
                                Image(.chevronRight)
                                    .frame(width: Heights.chevronLogElementWidth)
                            } else {
                                Spacer()
                                    .frame(width: Heights.chevronLogElementWidth)
                            }
                        }
                    }
                    if let displayValue = viewModel.renderable.displayValue, !displayValue.isEmpty {
                        Text(displayValue)
                            .foregroundColor(.textAndIconsSecondary)
                            .font(PrimaryFont.bodyM.font)
                            .padding(.top, Spacing.small)
                            .padding(.trailing, Spacing.large)
                    }
                    if let additionalValue = viewModel.renderable.additionalValue, !additionalValue.isEmpty {
                        Text(additionalValue)
                            .foregroundColor(.textAndIconsSecondary)
                            .font(PrimaryFont.captionM.font)
                            .padding(.top, Spacing.extraExtraSmall)
                            .padding(.trailing, Spacing.large)
                    }
                }
                .foregroundColor(.textAndIconsTertiary)
                .padding(.vertical, Spacing.small)
            }
        }
        .padding(.leading, Spacing.large)
        .contentShape(Rectangle())
    }
}

extension LogEntryView {
    final class ViewModel: ObservableObject {
        let renderable: LogEntryRenderable

        init(renderable: LogEntryRenderable) {
            self.renderable = renderable
        }
    }
}

#if DEBUG
    struct LogEntryView_Previews: PreviewProvider {
        static var previews: some View {
            VStack(spacing: 0) {
                LogEntryView(
                    viewModel: .init(
                        renderable: .init(
                            title: "Generated signature",
                            displayValue: "some value",
                            additionalValue: nil,
                            isWarning: false,
                            type: .fullDetails,
                            dateHeader: "Dec 09",
                            timestamp: "13:42",
                            navigationDetails: 0
                        )
                    )
                )
                LogEntryView(
                    viewModel: .init(
                        renderable: .init(
                            title: "Generated signature",
                            displayValue: nil,
                            additionalValue: nil,
                            isWarning: false,
                            type: .basic,
                            dateHeader: nil,
                            timestamp: "13:42",
                            navigationDetails: 0
                        )
                    )
                )
                LogEntryView(
                    viewModel: .init(
                        renderable: .init(
                            title: "Generated signature",
                            displayValue: "Very bad message",
                            additionalValue: nil,
                            isWarning: true,
                            type: .fullDetails,
                            dateHeader: nil,
                            timestamp: "13:42",
                            navigationDetails: 0
                        )
                    )
                )
                LogEntryView(
                    viewModel: .init(
                        renderable: .init(
                            title: "Generated signature with extremely long message that won't fit into single line",
                            displayValue: "Very bad message",
                            additionalValue: nil,
                            isWarning: true,
                            type: .basic,
                            dateHeader: "Dec 09",
                            timestamp: "13:42",
                            navigationDetails: 0
                        )
                    )
                )
            }
        }
    }
#endif
